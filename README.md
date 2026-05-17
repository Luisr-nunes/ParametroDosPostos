# ParametroDosPostos

Sistema para monitoramento e análise de parâmetros de postos de combustível no Brasil.

## 🏗️ Estrutura do Projeto

```
.
├── api/              # API REST (Axum) - localhost:3000
├── core_db/          # Camada de banco de dados compartilhada
├── scraper_anp/      # Web scraper + ingestão ANP
├── parser_pmqc/      # Parser de dados PMQC
├── frontend/         # Interface web (React + Vite + Leaflet)
├── migrations/       # Scripts SQL (init + seed)
├── data/             # Dados de entrada (CSVs, não versionados)
└── docs/             # Documentação e estratégia
```

## 🚀 Quick Start

### Pré-requisitos
- Rust 1.70+
- Node.js 18+
- PostgreSQL 13+ com PostGIS
- Docker (opcional)

### Setup Local

```bash
# 1. Configurar variáveis de ambiente
cp .env.example .env
# Edite o .env com suas credenciais

# 2. Criar database (com Docker)
docker compose up -d

# 3. Rodar migrations (se não usar Docker)
psql -U admin -d parametrodospostos -f migrations/001_init.sql

# 4. Build Rust
cargo build

# 5. Seed data (opcional — dados de teste)
cargo run --bin seed

# 6. Ingestão de dados reais (escolha um)
cargo run --bin ingest_master   # Via API oficial da ANP
cargo run --bin ingest_postos   # Via CSV da ANP

# 7. Rodar API
cargo run --package api

# 8. Frontend (novo terminal)
cd frontend
npm install
npm run dev
```

A API estará em `http://localhost:3000` e o frontend em `http://localhost:5173`.

## 📡 API Endpoints

```bash
# Listar postos (limit 50)
GET /api/postos

# Buscar postos por nome, CNPJ ou cidade
GET /api/postos/search?q=combustivel

# Health check
GET /health
```

## 🔧 Desenvolvimento

```bash
# Watch mode (Vite frontend)
cd frontend && npm run dev

# Build production
cargo build --release
cd frontend && npm run build

# Format code
cargo fmt

# Lint
cargo clippy
```

## 📊 Database Schema

- `postos` — Cadastro dos postos de combustível (com PostGIS)
- `interdicoes_anp` — Histórico de interdições da ANP
- `inspecoes_pmqc` — Resultados de inspeções PMQC

Veja `migrations/` para os scripts SQL.

## 📝 Dados de Entrada

Os CSVs de dados ficam na pasta `data/` (não versionados):
- `dados_postos.csv` — Cadastro dos revendedores varejistas
- `interdicoes.csv` — Medidas cautelares / interdições

## 🛠️ Stack

- **Backend**: Rust (Axum, SQLx, Tokio)
- **Frontend**: React + TypeScript + Vite + Leaflet
- **Database**: PostgreSQL + PostGIS
- **Ingestão**: Polars, Reqwest

---

**Última atualização**: 2026-05-17