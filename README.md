# Parâmetro dos Postos

<!-- badges -->
![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white) ![Axum](https://img.shields.io/badge/Axum-4A4A4A?style=flat-square) ![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=flat-square&logo=postgresql&logoColor=white) ![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=flat-square&logo=typescript&logoColor=white) ![React](https://img.shields.io/badge/React-61DAFB?style=flat-square&logo=react&logoColor=black) ![Vite](https://img.shields.io/badge/Vite-646CFF?style=flat-square&logo=vite&logoColor=white) ![Leaflet](https://img.shields.io/badge/Leaflet-199900?style=flat-square&logo=leaflet&logoColor=white) ![Tauri](https://img.shields.io/badge/Tauri-24C8D8?style=flat-square&logo=tauri&logoColor=white) ![Docker](https://img.shields.io/badge/Docker-2496ED?style=flat-square&logo=docker&logoColor=white)

[Read in English](README.en.md)

Radar de conformidade dos postos de combustível do Brasil: reúne o cadastro oficial da ANP, o histórico de interdições e os resultados das análises de qualidade do PMQC em um banco geoespacial, com API e mapa interativo para consulta.

## Sobre

O projeto cruza bases públicas da ANP para responder se um posto está regular e como foi seu histórico de fiscalização. Os dados são coletados por rotinas de ingestão em Rust, gravados em PostgreSQL com PostGIS e expostos por uma API consumida por um front-end com mapa.

A estratégia de evolução, de validador de planilhas para radar nacional de conformidade, está em [`docs/ESTRATEGIA_EVOLUCAO.md`](docs/ESTRATEGIA_EVOLUCAO.md).

## Funcionalidades

- Ingestão do cadastro de revendedores pela API pública da ANP, por UF (`ingest_master`), ou a partir do CSV oficial (`ingest_postos`).
- Parser do PMQC (Programa de Monitoramento da Qualidade dos Combustíveis), que baixa o JSON mensal e grava os ensaios de cada amostra.
- Banco com geolocalização dos postos (PostGIS).
- API para listar postos e buscar por nome, CNPJ ou cidade.
- Mapa interativo com Leaflet e marcadores por situação do posto.

## Arquitetura

```
API ANP / CSV ──► scraper_anp ──┐
JSON PMQC ──────► parser_pmqc ──┼──► core_db ──► PostgreSQL + PostGIS
                                │
                         api (Axum, :3000) ──► frontend (React + Leaflet)
```

## Tecnologias

| Camada | Tecnologia |
| --- | --- |
| Back-end | Rust (workspace), Axum 0.8, Tokio, SQLx 0.8 |
| Ingestão | Reqwest, Scraper, Polars |
| Banco | PostgreSQL 15 com PostGIS 3.3 (Docker) |
| Front-end | React 19, TypeScript, Vite, Leaflet / React Leaflet |
| Desktop (em configuração) | Tauri 2 |

## Estrutura

```
ParametroDosPostos/
├── api/            # API REST (Axum)
├── core_db/        # conexão e operações de banco compartilhadas; bin seed
├── scraper_anp/    # ingestão da ANP: bins ingest_master e ingest_postos
├── parser_pmqc/    # ingestão do PMQC
├── frontend/       # React + Vite (e projeto Tauri em src-tauri/)
├── migrations/     # 001_init.sql e 002_seed_data.sql
├── docs/
└── docker-compose.yml
```

Tabelas: `postos`, `interdicoes_anp` e `inspecoes_pmqc`.

## Como executar

Pré-requisitos: Rust estável, Node.js 18+ e Docker (ou PostgreSQL com PostGIS).

```bash
git clone https://github.com/Luisr-nunes/ParametroDosPostos.git
cd ParametroDosPostos

cp .env.example .env              # ajuste DATABASE_URL
docker compose up -d              # PostgreSQL + PostGIS
psql "$DATABASE_URL" -f migrations/001_init.sql

cargo build
cargo run --bin seed              # opcional: dados de teste
cargo run --bin ingest_master     # postos pela API da ANP
cargo run --package parser_pmqc   # análises do PMQC

cargo run --package api           # http://localhost:3000
```

Em outro terminal:

```bash
cd frontend
npm install
npm run dev                       # http://localhost:5173
```

Para usar uma API em outro endereço, defina `VITE_API_URL`. Os CSVs usados por `ingest_postos` ficam em `data/`, que não é versionada.

## API

| Método | Rota | Descrição |
| --- | --- | --- |
| GET | `/health` | Verificação de saúde |
| GET | `/api/postos` | Lista postos (até 50) |
| GET | `/api/postos/search?q=` | Busca por nome, CNPJ ou cidade |

## Desenvolvimento

```bash
cargo fmt
cargo clippy
cargo build --release
cd frontend && npm run build
```

## Autor

Luís Nunes — [GitHub](https://github.com/Luisr-nunes) · [LinkedIn](https://www.linkedin.com/in/luis-nunes-dev/)
