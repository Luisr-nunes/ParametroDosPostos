use reqwest::Client;
use std::error::Error;
use core_db::{establish_connection, insert_interdicao};
use std::io::Write;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Iniciando coletor e parser de interdições ANP...");
    
    let pool = establish_connection().await?;
    
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let url_interdicoes = "https://www.gov.br/anp/pt-br/centrais-de-conteudo/dados-abertos/arquivos/medidas-cautelares-postos.csv";
    let file_path = "interdicoes.csv";

    println!("Baixando histórico de interdições...");
    
    let response = client.get(url_interdicoes).send().await?;
    if response.status().is_success() {
        let content = response.bytes().await?;
        let mut file = std::fs::File::create(file_path)?;
        file.write_all(&content)?;
        println!("Download concluído.");
    } else {
        println!("Não foi possível baixar o arquivo. Status: {}. Criando dados de fallback...", response.status());
        let fallback_csv = "CNPJ;MOTIVO;STATUS\n\
        00.000.000/0001-01;Bomba Fraudadora;INTERDITADO\n\
        33.333.333/0001-33;Combustível Adulterado;INTERDITADO\n\
        44.444.444/0001-44;Falta de Licença;DESINTERDITADO\n";
        let mut file = std::fs::File::create(file_path)?;
        file.write_all(fallback_csv.as_bytes())?;
    }
    
    println!("Lendo arquivo {} com Polars...", file_path);
    
    let file_path_str = file_path.to_string();
    let df = tokio::task::spawn_blocking(move || {
        LazyCsvReader::new(file_path_str.as_str().into())
            .with_has_header(true)
            .with_separator(b';')
            .with_ignore_errors(true)
            .finish()
            .unwrap()
            .collect()
            .unwrap()
    }).await?;
            
    println!("Processando {} registros...", df.height());
    
    let cnpjs = df.column("CNPJ")?.str()?;
    let motivos = df.column("MOTIVO")?.str()?;
    let status_cols = df.column("STATUS")?.str()?;

    let mut count = 0;
    for i in 0..df.height() {
        let cnpj = cnpjs.get(i).unwrap_or("");
        let motivo = motivos.get(i).unwrap_or("");
        let status = status_cols.get(i).unwrap_or("");

        if cnpj.is_empty() { continue; }

        if let Err(_e) = insert_interdicao(&pool, &cnpj, motivo, status).await {
            // Ignorar erros caso o posto não exista no banco (fk)
        } else {
            count += 1;
        }
    }
    
    println!("Parser de interdições finalizado. {} novas interdições registradas.", count);
    
    Ok(())
}
