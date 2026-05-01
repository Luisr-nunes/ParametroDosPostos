use core_db::{establish_connection};
use polars::prelude::*;
use std::env;
use std::io::Write;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando Ingestão de Postos ANP...");
    
    let file_path = env::var("ANP_CSV_PATH").unwrap_or_else(|_| "dados_postos.csv".to_string());
    let download_url = "https://www.gov.br/anp/pt-br/centrais-de-conteudo/dados-abertos/arquivos/arquivos-dados-cadastrais-dos-revendedores-varejistas-de-combustiveis-automotivos/dados-cadastrais-revendedores-varejistas-combustiveis-automoveis.csv";

    println!("Baixando dados mais recentes da ANP em: {}...", download_url);
    
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let response = client.get(download_url).send().await?;
    
    if response.status().is_success() {
        let content = response.bytes().await?;
        let mut file = std::fs::File::create(&file_path)?;
        file.write_all(&content)?;
        println!("Download concluído com sucesso. Arquivo salvo em: {}", file_path);
    } else {
        println!("Falha ao baixar arquivo da ANP. Status: {}", response.status());
        if !std::path::Path::new(&file_path).exists() {
            println!("Erro crítico: Arquivo local não encontrado e download falhou.");
            return Ok(());
        }
        println!("Usando arquivo local existente para prosseguir.");
    }

    let file_path_clone = file_path.clone();
    let df = tokio::task::spawn_blocking(move || {
        LazyCsvReader::new(file_path_clone.as_str().into())
            .with_separator(b';')
            .with_has_header(true)
            .with_infer_schema_length(Some(100))
            .with_ignore_errors(true)
            .finish()
            .unwrap()
            .collect()
            .unwrap()
    }).await?;

    println!("CSV carregado. Total de postos encontrados: {}", df.height());

    let pool = establish_connection().await?;
    
    let cnpjs = df.column("CNPJ")?.str()?;
    let razoes = df.column("RAZAOSOCIAL")?.str()?;
    let enderecos = df.column("ENDERECO")?.str()?;
    let municipios = df.column("MUNICIPIO")?.str()?;
    let ufs = df.column("UF")?.str()?;
    // The dataset might not have "STATUS", let's mock it as ATIVO for now.
    // let status = df.column("STATUS")?.str()?;

    let mut inserted = 0;

    for i in 0..df.height() {
        let cnpj = cnpjs.get(i).unwrap_or("");
        let razao = razoes.get(i).unwrap_or("");
        let endereco = enderecos.get(i).unwrap_or("");
        let municipio = municipios.get(i).unwrap_or("");
        let uf = ufs.get(i).unwrap_or("");
        let stat = "ATIVO"; // hardcoded since we lack the column in the current dataset

        if cnpj.is_empty() { continue; }

        let result = sqlx::query(
            "INSERT INTO postos (cnpj, razao_social, endereco, municipio, uf, status_autorizacao) \
             VALUES ($1, $2, $3, $4, $5, $6) \
             ON CONFLICT (cnpj) DO UPDATE SET \
                razao_social = EXCLUDED.razao_social, \
                status_autorizacao = EXCLUDED.status_autorizacao"
        )
        .bind(cnpj)
        .bind(razao)
        .bind(endereco)
        .bind(municipio)
        .bind(uf)
        .bind(stat)
        .execute(&pool)
        .await;

        if result.is_ok() {
            inserted += 1;
            if inserted % 1000 == 0 {
                println!("{} postos inseridos/atualizados...", inserted);
            }
        }
    }

    println!("Ingestão Nacional concluída! Total inserido no PostGIS: {}", inserted);
    Ok(())
}
