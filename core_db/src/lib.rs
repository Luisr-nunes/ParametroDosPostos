use sqlx::{postgres::PgPoolOptions, Pool, Postgres, FromRow};
use serde::{Deserialize, Serialize};

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:password123@localhost/parametrodospostos".to_string());

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Interdicao {
    pub motivo: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pmqc {
    pub parametro: String,
    pub conforme: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PostoCompleto {
    pub cnpj: String,
    pub razao_social: String,
    pub endereco: String,
    pub municipio: String,
    pub status_autorizacao: Option<String>,
    pub interdicoes: Vec<Interdicao>,
    pub pmqc: Vec<Pmqc>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

pub async fn get_postos_completos(pool: &Pool<Postgres>) -> Result<Vec<PostoCompleto>, sqlx::Error> {
    let postos_raw = sqlx::query_as::<_, PostoCompletoRaw>(
        "SELECT cnpj, razao_social, endereco, municipio, status_autorizacao, ST_Y(localizacao::geometry) as latitude, ST_X(localizacao::geometry) as longitude FROM postos LIMIT 50"
    )
    .fetch_all(pool)
    .await?;

    let mut resultados = Vec::new();

    for p in postos_raw {
        let interdicoes = sqlx::query_as::<_, Interdicao>(
            "SELECT motivo, status FROM interdicoes_anp WHERE posto_cnpj = $1"
        )
        .bind(&p.cnpj)
        .fetch_all(pool)
        .await?;

        let pmqc = sqlx::query_as::<_, Pmqc>(
            "SELECT parametro, conforme FROM inspecoes_pmqc WHERE posto_cnpj = $1"
        )
        .bind(&p.cnpj)
        .fetch_all(pool)
        .await?;

        resultados.push(PostoCompleto {
            cnpj: p.cnpj.clone(),
            razao_social: p.razao_social,
            endereco: p.endereco,
            municipio: p.municipio,
            status_autorizacao: p.status_autorizacao,
            interdicoes,
            pmqc,
            latitude: p.latitude,
            longitude: p.longitude,
        });
    }

    Ok(resultados)
}

#[derive(FromRow)]
struct PostoCompletoRaw {
    pub cnpj: String,
    pub razao_social: String,
    pub endereco: String,
    pub municipio: String,
    pub status_autorizacao: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

pub async fn search_postos(pool: &Pool<Postgres>, search_query: &str) -> Result<Vec<PostoCompleto>, sqlx::Error> {
    let like_query = format!("%{}%", search_query);

    let postos_raw = sqlx::query_as::<_, PostoCompletoRaw>(
        "SELECT cnpj, razao_social, endereco, municipio, status_autorizacao, ST_Y(localizacao::geometry) as latitude, ST_X(localizacao::geometry) as longitude FROM postos WHERE razao_social ILIKE $1 OR cnpj ILIKE $1 OR municipio ILIKE $1 LIMIT 30"
    )
    .bind(like_query)
    .fetch_all(pool)
    .await?;

    let mut resultados = Vec::new();

    for p in postos_raw {
        let interdicoes = sqlx::query_as::<_, Interdicao>(
            "SELECT motivo, status FROM interdicoes_anp WHERE posto_cnpj = $1"
        )
        .bind(&p.cnpj)
        .fetch_all(pool)
        .await?;

        let pmqc = sqlx::query_as::<_, Pmqc>(
            "SELECT parametro, conforme FROM inspecoes_pmqc WHERE posto_cnpj = $1"
        )
        .bind(&p.cnpj)
        .fetch_all(pool)
        .await?;

        resultados.push(PostoCompleto {
            cnpj: p.cnpj.clone(),
            razao_social: p.razao_social,
            endereco: p.endereco,
            municipio: p.municipio,
            status_autorizacao: p.status_autorizacao,
            interdicoes,
            pmqc,
            latitude: p.latitude,
            longitude: p.longitude,
        });
    }

    Ok(resultados)
}

pub async fn insert_interdicao(
    pool: &Pool<Postgres>,
    cnpj: &str,
    motivo: &str,
    status: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO interdicoes_anp (posto_cnpj, motivo, status, data_interdicao) VALUES ($1, $2, $3, CURRENT_DATE)"
    )
    .bind(cnpj)
    .bind(motivo)
    .bind(status)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_pmqc(
    pool: &Pool<Postgres>,
    cnpj: &str,
    combustivel: &str,
    parametro: &str,
    conforme: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO inspecoes_pmqc (posto_cnpj, combustivel, parametro, conforme, data_coleta) VALUES ($1, $2, $3, $4, CURRENT_DATE)"
    )
    .bind(cnpj)
    .bind(combustivel)
    .bind(parametro)
    .bind(conforme)
    .execute(pool)
    .await?;
    Ok(())
}
