//! Parser de dados do PMQC (Programa de Monitoramento da Qualidade dos Combustíveis).
//!
//! Baixa o JSON mensal do PMQC do portal de dados abertos da ANP,
//! extrai os ensaios realizados em cada amostra e insere no banco de dados.
//!
//! Uso: `cargo run --package parser_pmqc`

use std::collections::HashMap;
use std::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use core_db::{establish_connection, insert_pmqc};

/// Estrutura raiz do JSON do PMQC — contém as UFs.
#[derive(Debug, Serialize, Deserialize)]
struct PMQCData {
    #[serde(rename = "UF")]
    ufs: HashMap<String, UFEntry>,
}

/// Entrada por UF — pode conter municípios ou metadados (ex: DataGeracao).
#[derive(Debug, Serialize, Deserialize)]
struct UFEntry {
    municipios: Option<HashMap<String, MunicipioData>>,
}

/// Dados de um município — contém as amostras coletadas.
#[derive(Debug, Serialize, Deserialize)]
struct MunicipioData {
    amostras: HashMap<String, AmostraData>,
}

/// Dados de uma amostra — produto, posto e ensaios realizados.
#[derive(Debug, Serialize, Deserialize)]
struct AmostraData {
    #[serde(rename = "Produto")]
    produto: String,
    #[serde(rename = "Posto")]
    posto: PostoData,
    #[serde(rename = "Ensaios")]
    ensaios: HashMap<String, EnsaioData>,
}

/// Identificação do posto na amostra.
#[derive(Debug, Serialize, Deserialize)]
struct PostoData {
    #[serde(rename = "CNPJ")]
    cnpj: String,
}

/// Resultado de um ensaio laboratorial.
#[derive(Debug, Serialize, Deserialize)]
struct EnsaioData {
    #[serde(rename = "Conforme")]
    conforme: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🔬 Iniciando parser PMQC (dados 2026)...");

    let pool = establish_connection().await?;
    println!("✅ Conexão com o banco de dados estabelecida.");
    
    let download_url = "https://www.gov.br/anp/pt-br/centrais-de-conteudo/dados-abertos/arquivos/pmqc/2026/pmqc_2026_01.json";

    println!("📥 Baixando dados do PMQC...");
    
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let response = client.get(download_url).send().await?;
    
    if !response.status().is_success() {
        eprintln!("❌ Falha ao baixar PMQC (Status: {}). Abortando.", response.status());
        return Ok(());
    }

    println!("📊 Download concluído. Parseando JSON (pode levar alguns segundos)...");
    let pmqc_json: PMQCData = response.json().await?;

    let mut ensaios_inseridos = 0;
    let mut amostras_processadas = 0;

    for (uf_name, uf_entry) in &pmqc_json.ufs {
        if let Some(municipios) = &uf_entry.municipios {
            println!("  📍 Processando UF: {} ({} municípios)...", uf_name, municipios.len());
            for (_mun_name, mun_data) in municipios {
                for (_amostra_id, amostra_data) in &mun_data.amostras {
                    amostras_processadas += 1;
                    let cnpj = &amostra_data.posto.cnpj;
                    let produto = &amostra_data.produto;

                    for (ensaio_name, ensaio_data) in &amostra_data.ensaios {
                        let conforme = ensaio_data.conforme.to_uppercase() == "SIM";

                        if insert_pmqc(&pool, cnpj, produto, ensaio_name, conforme).await.is_ok() {
                            ensaios_inseridos += 1;
                        }
                        // Silenciosamente ignora erros de postos não cadastrados (FK constraint)
                    }
                    
                    if amostras_processadas % 1000 == 0 {
                        println!("  📈 {} amostras / {} ensaios inseridos...", amostras_processadas, ensaios_inseridos);
                    }
                }
            }
        } else {
            println!("  ⏭️  Ignorando metadados na chave: {}", uf_name);
        }
    }
    
    println!("🎉 Parser PMQC finalizado!");
    println!("   Amostras processadas: {}", amostras_processadas);
    println!("   Ensaios inseridos:    {}", ensaios_inseridos);
    Ok(())
}
