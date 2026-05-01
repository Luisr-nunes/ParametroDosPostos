use axum::{
    routing::get,
    Router,
    Json,
    extract::{State, Query},
};
use std::net::SocketAddr;
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
}

#[tokio::main]
async fn main() {
    let pool = core_db::establish_connection().await.expect("Failed to connect to DB");

    // Habilitando CORS para o React/Tauri conseguir consumir sem block
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/api/postos", get(list_postos))
        .route("/api/postos/search", get(search_postos))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn list_postos(State(pool): State<Pool<Postgres>>) -> Json<Vec<core_db::PostoCompleto>> {
    let postos = core_db::get_postos_completos(&pool).await.unwrap_or_default();
    Json(postos)
}

async fn search_postos(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<SearchParams>,
) -> Json<Vec<core_db::PostoCompleto>> {
    let query = params.q.unwrap_or_default();
    if query.trim().is_empty() {
        return Json(vec![]);
    }
    let postos = core_db::search_postos(&pool, &query).await.unwrap_or_default();
    Json(postos)
}
