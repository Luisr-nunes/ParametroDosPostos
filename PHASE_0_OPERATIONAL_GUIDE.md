# 🚀 GUIA DE OPERAÇÃO - PHASE 0 (Preparação)
## ParametroDosPostos - Primeira Sprint

**Status**: Ready to START  
**Duration**: 2 semanas (10 dias úteis)  
**Owner**: Dev1 (primary), Dev2 (support)  

---

## 📅 TIMELINE SEMANAL

### Semana 1: Infrastructure Setup

```
SEGUNDA (2 horas)     TERÇA (3 horas)       QUARTA (2 horas)
├─ Fix Cargo.toml     ├─ Setup logging      ├─ Create .env.example
├─ Verify build       ├─ Setup tests        ├─ Document config vars
└─ Create branch      └─ Create test suite  └─ Health endpoint

QUINTA (2.5 horas)    SEXTA (2 horas)
├─ GitHub Actions CI  ├─ Team review
├─ Verify workflow    ├─ Fix issues
└─ PR ready           └─ Get approvals
```

### Semana 2: Finalization

```
SEGUNDA (1 hora)      TERÇA-SEXTA (monitoring)
├─ Merge to develop   ├─ Verify CI/CD stability
├─ Deploy staging     ├─ Monitor for issues
└─ Documentation      └─ Be ready for Phase 1 kick-off
```

---

## 📝 TAREFAS DETALHADAS

### Tarefa 0.1: Atualizar Cargo.toml (30 min)

**Objetivo**: Corrigir edition de 2024 → 2021 em todos os crates

**Passos**:

```bash
# 1. Ir para projeto root
cd /path/to/ParametroDosPostos

# 2. Verificar problema atual
grep "edition" */Cargo.toml
# Output esperado:
# api/Cargo.toml:edition = "2024"
# core_db/Cargo.toml:edition = "2024"
# ... (todas mostram 2024)

# 3. Corrigir
for file in {api,core_db,scraper_anp,parser_pmqc}/Cargo.toml; do
  sed -i 's/edition = "2024"/edition = "2021"/g' "$file"
done

# 4. Verificar resultado
grep "edition" */Cargo.toml
# Output esperado:
# api/Cargo.toml:edition = "2021"
# core_db/Cargo.toml:edition = "2021"
# ... (todas mostram 2021)

# 5. Verificar que build passa
cargo check --all
# Esperado: Compiling ... Finished `dev` profile

# 6. Commit
git add -A
git commit -m "[PREP] fix: Update Cargo.toml edition to 2021"
```

**Validação**: ✅ `cargo check --all` passa sem erros

**Tempo Estimado**: 30 minutos  
**Owner**: Dev1  

---

### Tarefa 0.2: Setup Logging Framework (2 horas)

**Objetivo**: Implementar structured logging para debug e monitoramento

**Passos**:

#### 2.1 Adicionar Dependencies

```bash
# Add tracing to all crates
cargo add -p core_db tracing tracing-subscriber serde_json
cargo add -p api tracing tracing-subscriber tracing-log
cargo add -p scraper_anp tracing tracing-subscriber
cargo add -p parser_pmqc tracing tracing-subscriber

# Verify Cargo.toml
grep -A2 "tracing" core_db/Cargo.toml
# Esperado: 
# [dependencies]
# tracing = "0.1"
# tracing-subscriber = "0.3"
```

#### 2.2 Implementar em core_db

```rust
// core_db/src/lib.rs - ADICIONAR NO TOPO
use tracing::{debug, error, warn, span, Level};
use std::time::Instant;

pub async fn establish_connection() -> Result<Pool<Postgres>> {
    let span = span!(Level::DEBUG, "establish_connection");
    let _enter = span.enter();
    
    debug!("Connecting to database...");
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/parametro_postos".to_string());
    
    debug!(url = &database_url[0..20], "Using database URL");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    debug!("Connection pool established");
    Ok(pool)
}

pub async fn get_postos_completos(pool: &Pool<Postgres>) -> Result<Vec<PostoCompleto>> {
    let span = span!(Level::DEBUG, "get_postos_completos");
    let _enter = span.enter();
    
    debug!("Fetching postos from database");
    let start = Instant::now();
    
    let postos = sqlx::query_as::<_, PostoRow>("SELECT * FROM postos LIMIT 50")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            error!("Database query failed: {}", e);
            e
        })?;
    
    let elapsed = start.elapsed();
    debug!(
        count = postos.len(),
        duration_ms = elapsed.as_millis(),
        "Query completed successfully"
    );
    
    Ok(postos)
}
```

#### 2.3 Implementar em API

```rust
// api/src/main.rs - ADICIONAR INIT
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .json()  // Structured JSON output
        .init();
    
    // Log startup
    tracing::info!("Starting API server...");
    
    // Rest of main function...
}
```

#### 2.4 Verificar Funcionamento

```bash
# Run com logging
RUST_LOG=debug cargo run --bin api

# Expected output (JSON structured):
# {"timestamp":"2026-05-01T10:00:00.123Z","level":"DEBUG","message":"Connecting to database...","target":"core_db"}
# {"timestamp":"2026-05-01T10:00:00.245Z","level":"DEBUG","message":"Connection pool established","target":"core_db"}
# {"timestamp":"2026-05-01T10:00:01.000Z","level":"INFO","message":"Starting API server...","target":"api"}

# Make request to test
curl http://localhost:3000/health

# Check logs for query times
# {"timestamp":"...","level":"DEBUG","message":"Fetching postos","duration_ms":150,...}
```

**Validação**: ✅ Logs aparecem em formato JSON quando app roda

**Tempo Estimado**: 2 horas  
**Owner**: Dev1  

---

### Tarefa 0.3: Criar .env.example (1 hora)

**Objetivo**: Documentar todas as variáveis de configuração

**Passos**:

```bash
# 1. Criar arquivo
cat > .env.example << 'EOF'
# ===== DATABASE =====
# PostgreSQL connection string
# Format: postgresql://[user[:password]@][host][:port]/database
DATABASE_URL=postgresql://user:password@localhost:5432/parametro_postos

# Maximum database connections
DATABASE_MAX_CONNECTIONS=5

# Connection timeout (seconds)
DATABASE_CONNECT_TIMEOUT=10

# ===== API =====
# API server port
API_PORT=3000

# API server host
API_HOST=0.0.0.0

# API timeout (seconds)
API_TIMEOUT=30

# ===== LOGGING =====
# Log level: error, warn, info, debug, trace
RUST_LOG=info

# JSON structured logs (true) or human readable (false)
LOG_JSON=true

# ===== FEATURES =====
# Enable optimized query path (Phase 1)
ENABLE_OPTIMIZED_QUERIES=false

# Enable feature flags system
ENABLE_FEATURE_FLAGS=true

# ===== ENVIRONMENT =====
# Environment: development, staging, production
ENVIRONMENT=development

# ===== MONITORING =====
# Prometheus metrics port
METRICS_PORT=9090

# Sentry DSN (error tracking, optional)
SENTRY_DSN=

# ===== SECURITY =====
# Rate limit: requests per minute per IP
RATE_LIMIT_PER_MINUTE=100

# JWT Secret (if using auth)
JWT_SECRET=your-secret-key-here
EOF

# 2. Copy para .env para desenvolvimento local
cp .env.example .env

# 3. Editar .env com valores locais
# (não commit .env, apenas .env.example)

# 4. Verificar que .gitignore tem .env
echo ".env" >> .gitignore
echo ".env.local" >> .gitignore

# 5. Commit
git add .env.example .gitignore
git commit -m "[PREP] docs: Create .env.example with all configuration"
```

**Validação**: 
- ✅ Arquivo .env.example criado e documentado
- ✅ .gitignore contem .env
- ✅ .env local pode carregar

**Tempo Estimado**: 1 hora  
**Owner**: Dev1  

---

### Tarefa 0.4: Setup Test Infrastructure (1.5 horas)

**Objetivo**: Criar estrutura básica de testes que pode ser expandida nas fases posteriores

**Passos**:

#### 4.1 Criar estrutura de pastas

```bash
# Create test directories
mkdir -p tests/integration
mkdir -p core_db/tests
mkdir -p api/tests

# Create fixtures directory
mkdir -p tests/fixtures
```

#### 4.2 Criar test file para core_db

```rust
// core_db/tests/integration_test.rs
#[tokio::test]
async fn test_establish_connection() {
    // TODO: Setup test database
    // This is a placeholder - will be expanded in Phase 1
    assert_eq!(1, 1);
}

#[test]
fn test_structs_compile() {
    // Basic test to verify types are correct
    // Expanded in Phase 1
    assert!(true);
}
```

#### 4.3 Criar test file para API

```rust
// api/tests/integration_test.rs
#[tokio::test]
async fn test_health_endpoint() {
    // TODO: Test health check endpoint
    // This is a placeholder - will be expanded in Phase 2
    assert_eq!(1, 1);
}
```

#### 4.4 Configurar Cargo.toml para testes

```toml
# core_db/Cargo.toml - Adicionar no final:
[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls"] }

# api/Cargo.toml - Adicionar no final:
[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
```

#### 4.5 Verificar que testes rodam

```bash
# Run all tests
cargo test --all

# Expected output:
# running 3 tests
# test test_establish_connection ... ok
# test test_structs_compile ... ok
# test test_health_endpoint ... ok
# 
# test result: ok. 3 passed; 0 failed
```

**Validação**: 
- ✅ `cargo test --all` passa
- ✅ Estrutura de testes em lugar

**Tempo Estimado**: 1.5 horas  
**Owner**: Dev1 + Dev2 (pair program)  

---

### Tarefa 0.5: Setup GitHub Actions CI/CD (3 horas)

**Objetivo**: Criar pipeline automatizado que valida código em cada commit

**Passos**:

#### 5.1 Criar workflow file

```bash
# Create directory
mkdir -p .github/workflows

# Create CI configuration
cat > .github/workflows/ci.yml << 'EOF'
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  check:
    runs-on: ubuntu-latest
    name: Check & Test
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      
      - uses: Swatinem/rust-cache@v2
      
      # Step 1: Check
      - name: Cargo check
        run: cargo check --all
      
      # Step 2: Format
      - name: Cargo fmt
        run: cargo fmt --all -- --check
      
      # Step 3: Clippy (linting)
      - name: Cargo clippy
        run: cargo clippy --all -- -D warnings
      
      # Step 4: Tests
      - name: Cargo test
        run: cargo test --all
  
  coverage:
    runs-on: ubuntu-latest
    name: Code Coverage
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      
      - name: Generate coverage
        run: cargo tarpaulin --out Xml
      
      # TODO: Upload to codecov in Phase 1
EOF

# Commit
git add .github/workflows/ci.yml
git commit -m "[PREP] ci: Add GitHub Actions CI/CD pipeline"
```

#### 5.2 Testar workflow localmente (opcional)

```bash
# Install act (local GitHub Actions runner)
# Requires Docker

# Run workflow locally
act

# Expected output:
# ⚙  Preparing environment...
# ⚙  Preparing container...
# ⚙  Running step...
# ✓ Cargo check completed
# ✓ Cargo fmt completed
# ✓ Cargo clippy completed
# ✓ Cargo test completed
```

#### 5.3 Push para GitHub e verificar

```bash
# 1. Create feature branch
git checkout -b feature/phase-0-prep develop

# 2. Make final commits if any
git add -A
git commit -m "[PREP] final: Phase 0 preparation complete"

# 3. Push branch
git push origin feature/phase-0-prep

# 4. Open PR on GitHub
# - Go to GitHub repo
# - Click "New Pull Request"
# - Select base: develop, compare: feature/phase-0-prep
# - Create PR

# 5. Wait for CI to run (5-10 min)
# - Go to PR page
# - See "Checks" section with workflow results

# Expected: ✅ All checks pass
```

**Validação**: 
- ✅ GitHub Actions runs on PR
- ✅ All checks pass (check, test, clippy, fmt)

**Tempo Estimado**: 3 horas  
**Owner**: Dev1  

---

### Tarefa 0.6: Health Check Endpoint (1 hora)

**Objetivo**: Criar endpoint simples /health para monitoring

**Passos**:

```rust
// api/src/main.rs - Adicionar handler

use axum::{
    Json,
    extract::State,
};
use serde_json::json;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::postgres::PgPool,
}

// Add route
async fn health(State(state): State<AppState>) -> Json<serde_json::Value> {
    match state.pool.acquire().await {
        Ok(_) => {
            Json(json!({
                "status": "ok",
                "database": "connected",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
        Err(_) => {
            Json(json!({
                "status": "error",
                "database": "disconnected",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
    }
}

// In main() router builder, add:
.route("/health", get(health))
```

**Test**:

```bash
# Start API
cargo run --bin api &

# Test endpoint
curl http://localhost:3000/health

# Expected response:
# {"status":"ok","database":"connected","timestamp":"2026-05-01T10:00:00Z"}
```

**Validação**: ✅ GET /health retorna JSON com status

**Tempo Estimado**: 1 hora  
**Owner**: Dev1  

---

## 📊 ACCEPTANCE CRITERIA

Phase 0 está **COMPLETO** quando:

### Technical
- [ ] ✅ Cargo.toml edition atualizado para 2021 em todos os crates
- [ ] ✅ Logging framework (tracing) funcional em todos os crates
- [ ] ✅ Logs estruturados em JSON format
- [ ] ✅ .env.example completo e documentado
- [ ] ✅ .env carrega corretamente via dotenvy (ou similar)
- [ ] ✅ Test infrastructure criado (tests/ pasta, basic tests)
- [ ] ✅ GitHub Actions workflow configurado
- [ ] ✅ CI passes: check, fmt, clippy, test
- [ ] ✅ Health endpoint implementado e funcionando

### Quality
- [ ] ✅ cargo check --all passes
- [ ] ✅ cargo fmt --all passes
- [ ] ✅ cargo clippy --all passes
- [ ] ✅ cargo test --all passes
- [ ] ✅ Zero compiler warnings

### Documentation
- [ ] ✅ .env.example documentado com todas as variáveis
- [ ] ✅ README atualizado com setup instructions
- [ ] ✅ Commit messages seguem convenção [PREP] ...

### Team
- [ ] ✅ 2 code reviewers aprovaram
- [ ] ✅ Tech Lead signed off
- [ ] ✅ Branch merged para develop
- [ ] ✅ No regressions em staging

---

## 🎯 SUCCESS METRICS (Phase 0)

| Métrica | Target | Current | Status |
|---------|--------|---------|--------|
| Build Time | <45s | TBD | ⏳ |
| Clippy Warnings | 0 | TBD | ⏳ |
| Test Pass Rate | 100% | TBD | ⏳ |
| CI Pipeline Time | <5 min | TBD | ⏳ |
| Documentation Complete | 100% | TBD | ⏳ |

---

## 🚨 COMMON ISSUES & FIXES

### Issue: "edition 2024 is unstable"

**Cause**: Cargo.toml still has edition = "2024"  
**Fix**: Run `sed -i 's/edition = "2024"/edition = "2021"/g' */Cargo.toml`

### Issue: "tracing not found in current scope"

**Cause**: Dependency not added to Cargo.toml  
**Fix**: `cargo add -p core_db tracing tracing-subscriber`

### Issue: "GitHub Actions workflow not triggering"

**Cause**: File not committed or branch not pushed  
**Fix**: `git add .github/workflows/ci.yml && git commit && git push`

### Issue: "Health endpoint returns 404"

**Cause**: Route not registered in router  
**Fix**: Add `.route("/health", get(health))` to router builder

---

## 📞 SUPPORT & ESCALATION

**Questions?** Slack #refactoring-phase-0  
**Blocker?** Escalate to @tech-lead  
**Technical help?** Tag @dev-lead  

---

## 🎬 NEXT: AFTER PHASE 0

Once Phase 0 is COMPLETE:
1. ✅ Merge develop into staging for 1-day validation
2. ✅ Do final safety checks
3. ✅ Schedule kick-off meeting for Phase 1
4. ✅ Dev1 starts Phase 1 (Core_DB) immediately
5. ✅ Dev2 reviews Phase 1 PR as it comes in

---

**Created**: 2026-05-01  
**Owner**: Dev1  
**Status**: READY TO START  
**Next Update**: When Phase 0 complete  
