# Parâmetro dos Postos

<!-- badges -->
![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white) ![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=flat-square&logo=typescript&logoColor=white) ![CSS3](https://img.shields.io/badge/CSS3-663399?style=flat-square&logo=css&logoColor=white) ![HTML5](https://img.shields.io/badge/HTML5-E34F26?style=flat-square&logo=html5&logoColor=white) ![Docker](https://img.shields.io/badge/Docker-2496ED?style=flat-square&logo=docker&logoColor=white) ![Axum](https://img.shields.io/badge/Axum-4A4A4A?style=flat-square) ![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=flat-square&logo=postgresql&logoColor=white) ![React](https://img.shields.io/badge/React-61DAFB?style=flat-square&logo=react&logoColor=black) ![Vite](https://img.shields.io/badge/Vite-646CFF?style=flat-square&logo=vite&logoColor=white) ![Leaflet](https://img.shields.io/badge/Leaflet-199900?style=flat-square&logo=leaflet&logoColor=white) ![Tauri](https://img.shields.io/badge/Tauri-24C8D8?style=flat-square&logo=tauri&logoColor=white)

[Ler em Português](README.md)

A compliance radar for Brazilian gas stations: it combines the official ANP (National Petroleum Agency) registry, the history of shutdown orders and the PMQC fuel quality test results in a geospatial database, with an API and an interactive map to query them.

## About

The project cross-references public ANP datasets to answer whether a station is compliant and what its inspection history looks like. Data is collected by ingestion jobs written in Rust, stored in PostgreSQL with PostGIS and exposed through an API consumed by a map-based front end.

The roadmap, from spreadsheet validator to a national compliance radar, is in [`docs/ESTRATEGIA_EVOLUCAO.md`](docs/ESTRATEGIA_EVOLUCAO.md) (Portuguese).

## Features

- Ingestion of the station registry from ANP's public API, per state (`ingest_master`), or from the official CSV (`ingest_postos`).
- PMQC (Fuel Quality Monitoring Program) parser that downloads the monthly JSON and stores each sample's test results.
- Database with station geolocation (PostGIS).
- API to list stations and search by name, CNPJ (company tax ID) or city.
- Interactive Leaflet map with markers by station status.

## Architecture

```
ANP API / CSV ──► scraper_anp ──┐
PMQC JSON ──────► parser_pmqc ──┼──► core_db ──► PostgreSQL + PostGIS
                                │
                         api (Axum, :3000) ──► frontend (React + Leaflet)
```

## Tech stack

| Layer | Technology |
| --- | --- |
| Back end | Rust (workspace), Axum 0.8, Tokio, SQLx 0.8 |
| Ingestion | Reqwest, Scraper, Polars |
| Database | PostgreSQL 15 with PostGIS 3.3 (Docker) |
| Front end | React 19, TypeScript, Vite, Leaflet / React Leaflet |
| Desktop (being set up) | Tauri 2 |

## Structure

```
ParametroDosPostos/
├── api/            # REST API (Axum)
├── core_db/        # shared connection and database operations; seed bin
├── scraper_anp/    # ANP ingestion: ingest_master and ingest_postos bins
├── parser_pmqc/    # PMQC ingestion
├── frontend/       # React + Vite (and the Tauri project in src-tauri/)
├── migrations/     # 001_init.sql and 002_seed_data.sql
├── docs/
└── docker-compose.yml
```

Tables: `postos`, `interdicoes_anp` and `inspecoes_pmqc`.

## Running

Requirements: stable Rust, Node.js 18+ and Docker (or PostgreSQL with PostGIS).

```bash
git clone https://github.com/Luisr-nunes/ParametroDosPostos.git
cd ParametroDosPostos

cp .env.example .env              # set DATABASE_URL
docker compose up -d              # PostgreSQL + PostGIS
psql "$DATABASE_URL" -f migrations/001_init.sql

cargo build
cargo run --bin seed              # optional: test data
cargo run --bin ingest_master     # stations from the ANP API
cargo run --package parser_pmqc   # PMQC tests

cargo run --package api           # http://localhost:3000
```

In another terminal:

```bash
cd frontend
npm install
npm run dev                       # http://localhost:5173
```

To point to an API elsewhere, set `VITE_API_URL`. The CSVs used by `ingest_postos` go in `data/`, which is not versioned.

## API

| Method | Route | Description |
| --- | --- | --- |
| GET | `/health` | Health check |
| GET | `/api/postos` | List stations (up to 50) |
| GET | `/api/postos/search?q=` | Search by name, CNPJ or city |

## Development

```bash
cargo fmt
cargo clippy
cargo build --release
cd frontend && npm run build
```

## Author

Luís Nunes — [GitHub](https://github.com/Luisr-nunes) · [LinkedIn](https://www.linkedin.com/in/luis-nunes-dev/)
