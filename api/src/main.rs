//! API REST para consulta e busca de postos de combustível.
//!
//! Endpoints disponíveis:
//! - `GET /health` — Health check
//! - `GET /api/postos` — Lista os primeiros 50 postos
//! - `GET /api/postos/search?q=<termo>` — Busca por nome, CNPJ ou município

use axum::{
    routing::get,
    Router,
    Json,
    extract::{State, Query},
};
use std::net::SocketAddr;
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;
use serde::{Deserialize, Serialize};

/// Parâmetros de busca aceitos na query string.
#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
}

/// Resposta do health check.
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[tokio::main]
async fn main() {
    let pool = core_db::establish_connection()
        .await
        .expect("❌ Falha ao conectar ao banco de dados. Verifique DATABASE_URL.");

    eprintln!("✅ Conexão com o banco de dados estabelecida.");

    // CORS permissivo para desenvolvimento (React/Tauri)
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/postos", get(list_postos))
        .route("/api/postos/search", get(search_postos))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    eprintln!("🚀 API rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("❌ Não foi possível vincular à porta 3000. Já está em uso?");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("❌ Erro fatal no servidor HTTP");
}

/// Health check — retorna status da API.
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "parametro-dos-postos-api",
    })
}

/// Lista os primeiros 50 postos cadastrados com interdições e inspeções PMQC.
async fn list_postos(State(pool): State<Pool<Postgres>>) -> Json<Vec<core_db::PostoCompleto>> {
    let postos = core_db::get_postos_completos(&pool).await.unwrap_or_default();
    Json(postos)
}

/// Busca postos por razão social, CNPJ ou município.
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
