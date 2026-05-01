use std::collections::HashMap;
use std::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use core_db::{establish_connection, insert_pmqc};

#[derive(Debug, Serialize, Deserialize)]
struct PMQCData {
    #[serde(rename = "UF")]
    ufs: HashMap<String, UFEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UFEntry {
    // Making municipios optional to handle metadata keys like "DataGeracao" 
    // that might appear inside the UF object
    municipios: Option<HashMap<String, MunicipioData>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MunicipioData {
    amostras: HashMap<String, AmostraData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AmostraData {
    #[serde(rename = "Produto")]
    produto: String,
    #[serde(rename = "Posto")]
    posto: PostoData,
    #[serde(rename = "Ensaios")]
    ensaios: HashMap<String, EnsaioData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostoData {
    #[serde(rename = "CNPJ")]
    cnpj: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EnsaioData {
    #[serde(rename = "Conforme")]
    conforme: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Iniciando parser PMQC (JSON 2026)...");

    let pool = establish_connection().await?;
    println!("Conexão com o banco de dados estabelecida.");
    
    let download_url = "https://www.gov.br/anp/pt-br/centrais-de-conteudo/dados-abertos/arquivos/pmqc/2026/pmqc_2026_01.json";

    println!("Baixando dados do PMQC em: {}...", download_url);
    
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let response = client.get(download_url).send().await?;
    
    if !response.status().is_success() {
        println!("Falha ao baixar PMQC. Status: {}", response.status());
        return Ok(());
    }

    println!("Download concluído. Iniciando parse do JSON (pode levar alguns segundos)...");
    let pmqc_json: PMQCData = response.json().await?;
    println!("Parse do JSON concluído.");

    let mut count = 0;
    let mut total_amostras = 0;

    for (uf_name, uf_entry) in pmqc_json.ufs {
        if let Some(municipios) = uf_entry.municipios {
            println!("Processando UF: {}...", uf_name);
            for (_mun_name, mun_data) in municipios {
                for (_amostra_id, amostra_data) in mun_data.amostras {
                    total_amostras += 1;
                    let cnpj = &amostra_data.posto.cnpj;
                    let produto = &amostra_data.produto;

                    for (ensaio_name, ensaio_data) in amostra_data.ensaios {
                        let conforme = ensaio_data.conforme.to_uppercase() == "SIM";

                        if let Err(_) = insert_pmqc(&pool, cnpj, produto, &ensaio_name, conforme).await {
                            // Silenciosamente ignorar erros de postos não cadastrados
                        } else {
                            count += 1;
                        }
                    }
                    
                    if total_amostras % 1000 == 0 {
                        println!("{} amostras processadas ({} ensaios inseridos)...", total_amostras, count);
                    }
                }
            }
        } else {
            println!("Ignorando chave de metadados ou entrada inválida na UF: {}", uf_name);
        }
    }
    
    println!("Parser PMQC finalizado.");
    println!("Total de amostras processadas: {}", total_amostras);
    println!("Total de ensaios inseridos: {}", count);
    Ok(())
}
