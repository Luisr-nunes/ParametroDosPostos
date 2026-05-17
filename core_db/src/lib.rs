//! Camada de acesso ao banco de dados para o projeto ParametroDosPostos.
//!
//! Fornece conexão, models e queries compartilhadas entre API, scrapers e parsers.

use sqlx::{postgres::PgPoolOptions, Pool, Postgres, FromRow};
use serde::{Deserialize, Serialize};

/// Estabelece conexão com o PostgreSQL usando a variável `DATABASE_URL`.
///
/// Se `DATABASE_URL` não estiver definida, utiliza um fallback local para desenvolvimento.
pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("⚠️  DATABASE_URL não definida. Usando fallback local de desenvolvimento.");
        "postgres://admin:password123@localhost/parametrodospostos".to_string()
    });

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

/// Dados de uma interdição da ANP.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Interdicao {
    pub motivo: String,
    pub status: String,
}

/// Resultado de uma inspeção PMQC (Programa de Monitoramento da Qualidade dos Combustíveis).
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pmqc {
    pub parametro: String,
    pub conforme: bool,
}

/// Dados completos de um posto de combustível, incluindo interdições e inspeções.
#[derive(Debug, Serialize, Deserialize)]
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

/// Representação interna (row do banco) antes de enriquecer com interdições e PMQC.
#[derive(FromRow)]
struct PostoRaw {
    pub cnpj: String,
    pub razao_social: String,
    pub endereco: String,
    pub municipio: String,
    pub status_autorizacao: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

/// Enriquece uma lista de postos brutos com suas interdições e inspeções PMQC.
///
/// Executa queries adicionais por posto (N+1). Para grandes volumes,
/// considerar migrar para JOINs ou batch queries.
async fn enrich_postos(
    pool: &Pool<Postgres>,
    postos_raw: Vec<PostoRaw>,
) -> Result<Vec<PostoCompleto>, sqlx::Error> {
    let mut resultados = Vec::with_capacity(postos_raw.len());

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
            cnpj: p.cnpj,
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

/// Retorna os primeiros 50 postos cadastrados com dados completos.
pub async fn get_postos_completos(pool: &Pool<Postgres>) -> Result<Vec<PostoCompleto>, sqlx::Error> {
    let postos_raw = sqlx::query_as::<_, PostoRaw>(
        "SELECT cnpj, razao_social, endereco, municipio, status_autorizacao, \
         ST_Y(localizacao::geometry) as latitude, ST_X(localizacao::geometry) as longitude \
         FROM postos LIMIT 50"
    )
    .fetch_all(pool)
    .await?;

    enrich_postos(pool, postos_raw).await
}

/// Busca postos por razão social, CNPJ ou município (ILIKE, até 30 resultados).
pub async fn search_postos(pool: &Pool<Postgres>, search_query: &str) -> Result<Vec<PostoCompleto>, sqlx::Error> {
    let like_query = format!("%{}%", search_query);

    let postos_raw = sqlx::query_as::<_, PostoRaw>(
        "SELECT cnpj, razao_social, endereco, municipio, status_autorizacao, \
         ST_Y(localizacao::geometry) as latitude, ST_X(localizacao::geometry) as longitude \
         FROM postos \
         WHERE razao_social ILIKE $1 OR cnpj ILIKE $1 OR municipio ILIKE $1 \
         LIMIT 30"
    )
    .bind(like_query)
    .fetch_all(pool)
    .await?;

    enrich_postos(pool, postos_raw).await
}

/// Insere uma interdição da ANP para o posto identificado pelo CNPJ.
pub async fn insert_interdicao(
    pool: &Pool<Postgres>,
    cnpj: &str,
    motivo: &str,
    status: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO interdicoes_anp (posto_cnpj, motivo, status, data_interdicao) \
         VALUES ($1, $2, $3, CURRENT_DATE)"
    )
    .bind(cnpj)
    .bind(motivo)
    .bind(status)
    .execute(pool)
    .await?;
    Ok(())
}

/// Insere um resultado de inspeção PMQC para o posto identificado pelo CNPJ.
pub async fn insert_pmqc(
    pool: &Pool<Postgres>,
    cnpj: &str,
    combustivel: &str,
    parametro: &str,
    conforme: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO inspecoes_pmqc (posto_cnpj, combustivel, parametro, conforme, data_coleta) \
         VALUES ($1, $2, $3, $4, CURRENT_DATE)"
    )
    .bind(cnpj)
    .bind(combustivel)
    .bind(parametro)
    .bind(conforme)
    .execute(pool)
    .await?;
    Ok(())
}
