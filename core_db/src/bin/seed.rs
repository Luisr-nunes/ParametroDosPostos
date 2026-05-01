use core_db::{establish_connection};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Iniciando seed do banco de dados...");
    let pool = establish_connection().await?;

    let postos = vec![
        ("00.000.000/0001-01", "Auto Posto Bandeira S/A", "Av. Paulista, 1000", "São Paulo", "SP", "ATIVO"),
        ("11.111.111/0001-11", "Posto Centro LTDA", "Rua Augusta, 200", "São Paulo", "SP", "ATIVO"),
        ("22.222.222/0001-22", "Posto Ipiranga Br", "Av. Brasil, 500", "Rio de Janeiro", "RJ", "INATIVO"),
        ("33.333.333/0001-33", "Comercial de Combustiveis Sul", "Rua 15 de Novembro, 10", "Curitiba", "PR", "ATIVO"),
        ("44.444.444/0001-44", "Posto Estrela do Norte", "Av. Norte, 300", "Recife", "PE", "ATIVO"),
    ];

    for p in postos {
        let exists: (i64,) = sqlx::query_as("SELECT count(*) FROM postos WHERE cnpj = $1")
            .bind(p.0)
            .fetch_one(&pool)
            .await?;

        if exists.0 == 0 {
            sqlx::query(
                "INSERT INTO postos (cnpj, razao_social, endereco, municipio, uf, status_autorizacao) VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(p.0)
            .bind(p.1)
            .bind(p.2)
            .bind(p.3)
            .bind(p.4)
            .bind(p.5)
            .execute(&pool)
            .await?;
            println!("Posto inserido: {}", p.1);
        }
    }

    // Inserir algumas interdições fictícias
    let interdicoes = vec![
        ("11.111.111/0001-11", "Bomba fraudada (vazão incorreta)", "INTERDITADO"),
        ("22.222.222/0001-22", "Qualidade do Etanol Adulterada", "DESINTERDITADO"),
    ];

    for i in interdicoes {
        sqlx::query(
            "INSERT INTO interdicoes_anp (posto_cnpj, motivo, data_interdicao, status) VALUES ($1, $2, CURRENT_DATE, $3)"
        )
        .bind(i.0)
        .bind(i.1)
        .bind(i.2)
        .execute(&pool)
        .await?;
        println!("Interdição registrada para o CNPJ: {}", i.0);
    }

    // Inserir inspeções PMQC
    let pmqc = vec![
        ("00.000.000/0001-01", "Gasolina Comum", "Teor de Etanol", "27%", "27%", true),
        ("11.111.111/0001-11", "Etanol Hidratado", "Massa Específica", "790", "805", false),
    ];

    for pq in pmqc {
        sqlx::query(
            "INSERT INTO inspecoes_pmqc (posto_cnpj, combustivel, parametro, resultado_obtido, limite_especificacao, conforme, data_coleta) VALUES ($1, $2, $3, $4, $5, $6, CURRENT_DATE)"
        )
        .bind(pq.0)
        .bind(pq.1)
        .bind(pq.2)
        .bind(pq.3)
        .bind(pq.4)
        .bind(pq.5)
        .execute(&pool)
        .await?;
        println!("Inspeção PMQC registrada para o CNPJ: {}", pq.0);
    }

    println!("Seed concluído com sucesso!");
    Ok(())
}
