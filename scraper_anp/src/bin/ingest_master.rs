//! Ingestão de postos via API oficial de revendedores da ANP.
//!
//! Consulta a API pública da ANP por UF e insere/atualiza os postos
//! no banco de dados PostgreSQL com suporte a geolocalização (PostGIS).
//!
//! Uso: `cargo run --bin ingest_master`

use std::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use core_db::establish_connection;

/// Resposta da API de revendedores da ANP.
#[derive(Debug, Serialize, Deserialize)]
struct ANPResponse {
    data: Vec<PostoANP>,
}

/// Dados de um posto retornado pela API da ANP.
#[derive(Debug, Serialize, Deserialize)]
struct PostoANP {
    #[serde(rename = "cnpj")]
    cnpj: String,
    #[serde(rename = "razaoSocial")]
    razao_social: String,
    #[serde(rename = "endereco")]
    endereco: String,
    #[serde(rename = "municipio")]
    municipio: String,
    #[serde(rename = "uf")]
    uf: String,
    #[serde(rename = "distribuidora")]
    bandeira: String,
    #[serde(rename = "latitude")]
    latitude: String,
    #[serde(rename = "longitude")]
    longitude: String,
}

/// Todas as UFs do Brasil para iteração.
const UFS: &[&str] = &[
    "AC", "AL", "AM", "AP", "BA", "CE", "DF", "ES", "GO", "MA",
    "MG", "MS", "MT", "PA", "PB", "PE", "PI", "PR", "RJ", "RN",
    "RO", "RR", "RS", "SC", "SE", "SP", "TO",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Iniciando Ingestão Master via API Oficial da ANP...");

    let pool = establish_connection().await?;
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let mut total_count = 0;

    for uf in UFS {
        println!("📥 Buscando postos da UF: {}...", uf);
        
        // Delay entre requests para evitar rate limit (403 Forbidden)
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let url = format!("https://revendedoresapi.anp.gov.br/v1/combustivel?uf={}", uf);
        
        let response = client.get(&url).send().await?;
        if !response.status().is_success() {
            eprintln!("  ⚠️  Falha ao buscar UF {} (Status: {}). Pulando...", uf, response.status());
            continue;
        }

        let anp_data: ANPResponse = response.json().await?;
        println!("  📊 {} postos encontrados em {}.", anp_data.data.len(), uf);

        for p in &anp_data.data {
            if p.cnpj.is_empty() { continue; }

            // Formatar CNPJ se a API retornar apenas dígitos (14 chars → XX.XXX.XXX/XXXX-XX)
            let formatted_cnpj = if p.cnpj.len() == 14 {
                format!("{}.{}.{}/{}-{}", 
                    &p.cnpj[0..2], &p.cnpj[2..5], &p.cnpj[5..8], &p.cnpj[8..12], &p.cnpj[12..14])
            } else {
                p.cnpj.clone()
            };

            let lat: Option<f64> = p.latitude.parse().ok();
            let lon: Option<f64> = p.longitude.parse().ok();

            sqlx::query(
                "INSERT INTO postos (cnpj, razao_social, endereco, municipio, uf, bandeira, status_autorizacao, localizacao) \
                 VALUES ($1, $2, $3, $4, $5, $6, 'ATIVO', \
                    CASE WHEN $7 IS NOT NULL AND $8 IS NOT NULL \
                         THEN ST_SetSRID(ST_MakePoint($8, $7), 4326) \
                         ELSE NULL END) \
                 ON CONFLICT (cnpj) DO UPDATE SET \
                    razao_social = EXCLUDED.razao_social, \
                    endereco = EXCLUDED.endereco, \
                    municipio = EXCLUDED.municipio, \
                    bandeira = EXCLUDED.bandeira, \
                    localizacao = COALESCE(EXCLUDED.localizacao, postos.localizacao), \
                    data_atualizacao = CURRENT_TIMESTAMP"
            )
            .bind(&formatted_cnpj)
            .bind(&p.razao_social)
            .bind(&p.endereco)
            .bind(&p.municipio)
            .bind(&p.uf)
            .bind(&p.bandeira)
            .bind(lat)
            .bind(lon)
            .execute(&pool)
            .await?;

            total_count += 1;
        }
        println!("  ✅ UF {} concluída. Total acumulado: {}", uf, total_count);
    }

    println!("🎉 Ingestão Master finalizada! {} postos cadastrados/atualizados.", total_count);
    Ok(())
}
