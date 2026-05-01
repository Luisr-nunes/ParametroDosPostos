# ParametroDosPostos

Sistema para monitoramento e análise de parâmetros de postos de combustível no Brasil.

## 🏗️ Estrutura do Projeto

```
.
├── api/              # API REST (Axum) - localhost:3000
├── core_db/          # Camada de banco de dados compartilhada
├── scraper_anp/      # Web scraper da ANP (dados públicos)
├── parser_pmqc/      # Parser de dados PMQC
├── frontend/         # Interface web (React + Vite)
└── db/               # Migrations e scripts SQL
```

## 🚀 Quick Start

### Pré-requisitos
- Rust 1.70+
- Node.js 18+
- PostgreSQL 13+
- Docker (opcional)

### Setup Local

```bash
# 1. Criar database (com Docker)
docker-compose up -d

# 2. Rodar migrations
psql -U postgres -d parametro_postos -f db/init.sql

# 3. Build Rust
cargo build

# 4. Seed data (opcional)
cargo run --bin seed --release

# 5. Rodar API
cargo run --bin ingest_master
cargo run --package api --release

# 6. Frontend (novo terminal)
cd frontend
npm install
npm run dev
```

A API estará em `http://localhost:3000` e o frontend em `http://localhost:5173`

## 📡 API Endpoints

```bash
# Listar todos os postos
GET /api/postos

# Buscar postos
GET /api/postos/search?q=combustivel&limit=50

# Ver status
GET /health
```

## 🔧 Desenvolvimento

```bash
# Watch mode (Vite frontend)
npm run dev

# Build production
cargo build --release
npm run build

# Format code
cargo fmt

# Lint
cargo clippy
```

## 📊 Database Schema

- `postos` - Dados dos postos
- `combustiveis` - Tipos e preços de combustível
- `interdicoes` - Histórico de interdições

Veja `db/` para migrations.

## 📝 Dados de Entrada

- `dados_postos.csv` - Dados brutos dos postos
- `interdicoes.csv` - Histórico de interdições

## 🛠️ Stack

- **Backend**: Rust (Axum, SQLx, Tokio)
- **Frontend**: React + TypeScript + Vite
- **Database**: PostgreSQL
- **Scraping**: Polars, Reqwest, Scraper

---

**Última atualização**: 2026-05-01