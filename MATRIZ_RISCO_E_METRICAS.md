# 📊 MATRIZ DE RISCO E MÉTRICAS
## ParametroDosPostos - Refactoring Execution

**Data**: 2026-05-01 | **Atualização**: Semanal

---

## 1. MATRIZ DE RISCO CONSOLIDADA

### 1.1 Risk Score Matrix (Visualização)

```
                    IMPACTO (Consequência)
                Low (1)    Medium (2)   High (3)  Critical (4)
            ┌─────────────┬─────────────┬──────────┬──────────┐
        5   │    5        │     10      │   15     │    20    │  TRÈS HIGH
PROB    │    │ Unlikely    │  (Avoid)    │(Monitor) │(Prepare) │
        │    ├─────────────┼─────────────┼──────────┼──────────┤
        3   │    3        │     6       │    9     │    12    │   MEDIUM
        │    │ Possible    │  (Accept)   │(Mitigate)│(Plan)    │
        │    ├─────────────┼─────────────┼──────────┼──────────┤
        1   │    1        │     2       │    3     │     4    │    LOW
            │ Rare        │  (Ignore)   │(Plan)    │(Urgent)  │
            └─────────────┴─────────────┴──────────┴──────────┘
              Score:        Ação Recomendada
              1-3           ✅ PROCEED (monitorar)
              4-6           ⚠️ CONDITIONAL (mitigações obrigatórias)
              9+            🔴 CRITICAL (aprovação antes de prosseguir)
```

### 1.2 Risk Scorecard por Fase

| FASE | RISCO TECH | RISCO OPS | PROB % | IMPACT (1-4) | SCORE | STATUS | MITIGAÇÕES |
|------|-----------|----------|--------|------------|-------|--------|-----------|
| **0: Prep** | 1 (✅ Baixo) | 1 (✅ Baixo) | 5% | 1 (Low) | 0.25 | 🟢 GO | - |
| **1: Core_DB** | 3 (🟠 Alto) | 4 (🔴 Critical) | 20% | 4 (Critical) | **3.2** | ⚠️ COND | Feature flags, shadow reads, extensive tests |
| **2: API** | 2 (🟡 Médio) | 2 (🟡 Médio) | 10% | 2 (Medium) | 0.4 | 🟢 GO | API versioning, backward compatibility |
| **3: Ingest** | 2 (🟡 Médio) | 3 (🟠 Alto) | 15% | 3 (High) | **1.35** | ✅ GO | Transactions, retry logic, monitoring |
| **4: Frontend** | 1 (✅ Baixo) | 2 (🟡 Médio) | 10% | 2 (Medium) | 0.4 | 🟢 GO | Component testing, error boundaries |
| **5: DevOps** | 2 (🟡 Médio) | 2 (🟡 Médio) | 8% | 2 (Medium) | 0.32 | 🟢 GO | Staging validation, runbooks |
| **6: Harden** | 1 (✅ Baixo) | 1 (✅ Baixo) | 5% | 1 (Low) | 0.125 | 🟢 GO | Load testing, security audit |

### 1.3 Risk Details por Fase Crítica

#### 🔴 FASE 1: CORE_DB (Score: 3.2 - CRITICAL)

**Identificação de Riscos Específicos**:

| ID | Risk | Cenário de Falha | Impacto | Prob | Score | Mitigação |
|----|----|-----------------|--------|------|-------|-----------|
| R1.1 | SQL Sintax Error | JOIN query fails compile | API não sobe | 10% | 0.4 | Code review + testing |
| R1.2 | Missing relations | LEFT JOIN não retorna dados corretos | UI mostra dados incompletos | 15% | 0.6 | Assert tests + staging validation |
| R1.3 | Performance worse | Query performance piora | API latência aumenta | 5% | 0.2 | Benchmarking before/after |
| R1.4 | Connection pool drain | Pool esgota durante queries lentas | Timeouts, API down | 20% | 0.8 | Pool size testing + metrics |
| R1.5 | Backward compat break | Old code expects old schema | Deploym breaking change | 10% | 0.4 | Feature flags, dual implementation |

**Threshold de Go-Live**:
- ✅ All 5 risks scored <0.5 OR
- ✅ All mitigations in place AND
- ✅ 48h staging validation passed

---

#### 🟠 FASE 3: INGESTION (Score: 1.35 - MEDIUM)

| ID | Risk | Cenário | Impacto | Mitigação |
|----|----|----------|---------|-----------|
| R3.1 | Deadlock | Multiple ingestions paralelos | Data loss | Serialize ingestions OR use row-level locking |
| R3.2 | FK Violation | CNPJ sem matching posto | Silent skip | Explicit error logging + alerting |
| R3.3 | Duplicate data | Retry w/o idempotency | Data duplication | ON CONFLICT DO UPDATE clause |
| R3.4 | Network timeout | Download fails mid-way | Incomplete data | Retry + checkpoint logic |

---

### 1.4 Risk Decision Matrix

**Quando proceder?**

```
┌─────────────────────────────────────────────────────────────┐
│ FASE | SCORE | DECISION      | CONDITION                    │
├─────────────────────────────────────────────────────────────┤
│  0   │ 0.25  │ ✅ GO         │ Start ASAP                  │
│  1   │ 3.2   │ ⚠️ CONDITIONAL│ IF all mitigations ready   │
│  2   │ 0.4   │ ✅ GO         │ After Phase 1 ready        │
│  3   │ 1.35  │ ✅ GO         │ Parallel com Phase 2       │
│  4   │ 0.4   │ ✅ GO         │ Independent                │
│  5   │ 0.32  │ ✅ GO         │ Before production          │
│  6   │ 0.125 │ ✅ GO         │ Final validation           │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. DASHBOARD DE MÉTRICAS

### 2.1 Métrica #1: Latência de Query (GET /postos)

**Baseline**: 2500ms (95th percentile)  
**Meta**: <250ms (94% redução)

**Como Medir**:
```sql
-- PostgreSQL: Enable query logging
ALTER SYSTEM SET log_min_duration_statement = 0;
SELECT pg_reload_conf();

-- Então rodar
EXPLAIN ANALYZE SELECT * FROM postos LEFT JOIN interdicoes_anp USING (cnpj);
```

**Prometheus Query**:
```
histogram_quantile(0.95, rate(http_request_duration_ms_bucket{endpoint="get_postos"}[5m]))
```

**Grafana Panel**:
```json
{
  "title": "API Latency (95p) - GET /postos",
  "targets": [{
    "expr": "histogram_quantile(0.95, rate(http_request_duration_ms_bucket[5m]))"
  }],
  "thresholds": [
    {"value": 250, "color": "green", "label": "TARGET"},
    {"value": 500, "color": "yellow"},
    {"value": 2500, "color": "red", "label": "BASELINE"}
  ],
  "unit": "ms"
}
```

**Checklist de Sucesso**:
- [ ] Query time consistently <250ms
- [ ] 95th percentile <250ms
- [ ] No query timeouts

---

### 2.2 Métrica #2: Query Count

**Baseline**: 101 queries por request (50 postos + N+1)  
**Meta**: 1 query

**Como Medir**:
```rust
// Em testes: mock DB com query counter
#[test]
fn test_query_count() {
    let mock_db = MockDatabase::with_query_capture();
    let postos = get_postos_completos(&mock_db).await.unwrap();
    
    assert_eq!(mock_db.query_count(), 1, "Expected 1 query, got {}", mock_db.query_count());
}
```

**Em produção**: Log cada query
```rust
RUST_LOG=debug cargo run --bin api
// Output: [DEBUG sqlx] query: SELECT ...
```

**Threshold**: ✅ Sucesso = Apenas 1 query por request

---

### 2.3 Métrica #3: Test Coverage

**Baseline**: 0%  
**Meta**: 80%+

**Como Medir**:
```bash
# Instalar
cargo install cargo-tarpaulin

# Rodar
cargo tarpaulin --out Html --output-dir coverage

# Resultado
open coverage/index.html  # Ver cobertura visualmente
```

**CI Integration**:
```yaml
# .github/workflows/test.yml
- name: Test coverage
  run: |
    cargo tarpaulin --out Xml --output-dir coverage
    # Fail se < 80%
    COVERAGE=$(grep -oP 'lines-valid="\K[^"]*' coverage/cobertura.xml)
    if [ "$COVERAGE" -lt 80 ]; then
      echo "Coverage $COVERAGE% < 80% target"
      exit 1
    fi
```

**Threshold**: ✅ Sucesso = Coverage > 80%

---

### 2.4 Métrica #4: API Response Time (50th percentile)

**Baseline**: ~2500ms  
**Meta**: ~150ms (94% redução)

**Prometheus**:
```
histogram_quantile(0.50, rate(http_request_duration_ms_bucket[5m]))
```

**Grafana Panel**:
- Green: <200ms
- Yellow: 200-500ms
- Red: >500ms

---

### 2.5 Métrica #5: Data Ingestion Success Rate

**Baseline**: ~70% (silent failures, unknown)  
**Meta**: 99%+

**Como Medir**:
```rust
// Em cada ingestão
pub struct IngestionStats {
    pub total: usize,
    pub inserted: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

fn calculate_success_rate(stats: &IngestionStats) -> f64 {
    (stats.inserted as f64 / stats.total as f64) * 100.0
}
```

**Publish**:
```rust
// Send to Prometheus
histogram!("ingestion_success_rate", success_rate);
```

**Alert** se < 95%:
```yaml
groups:
- name: ingestion
  rules:
  - alert: HighIngestionErrorRate
    expr: ingestion_success_rate < 95
    for: 5m
    annotations:
      summary: "Ingestion error rate {{ $value }}%"
```

---

### 2.6 Métrica #6: Code Duplication

**Baseline**: ~15%  
**Meta**: <5%

**Como Medir**:
```bash
# Usar ferramenta de análise (manual ou automática)
# Procurar patterns duplicados
grep -r "parse_csv" src/  # Aparece 3 vezes = oportunidade consolidar
grep -r "insert_batch" src/  # Aparece 2 vezes

# Usar ferramenta: Radon (Python)
pip install radon
radon cc src/ -a  # Complexity check
```

**Manuais**: 
- Procurar por funções similares
- Procurar por blocks duplicados
- Consolidar em shared utils

---

### 2.7 Dashboard Template Completo

```json
{
  "dashboard": {
    "title": "ParametroDosPostos - Refactoring Progress",
    "refresh": "30s",
    "time": {"from": "now-30d", "to": "now"},
    "panels": [
      {
        "id": 1,
        "title": "API Latency (95p) - Phase 1 Target",
        "query": "histogram_quantile(0.95, rate(http_request_duration_ms_bucket[5m]))",
        "unit": "ms",
        "targets": [250, 500, 2500],
        "type": "gauge"
      },
      {
        "id": 2,
        "title": "Query Count per Request",
        "query": "avg(rate(db_queries_total[1m]))",
        "unit": "queries",
        "targets": [1, 10, 101],
        "type": "gauge"
      },
      {
        "id": 3,
        "title": "Test Coverage (core_db)",
        "query": "code_coverage_percent{module='core_db'}",
        "unit": "%",
        "targets": [80, 60, 40],
        "type": "gauge"
      },
      {
        "id": 4,
        "title": "Ingestion Success Rate",
        "query": "ingestion_success_rate",
        "unit": "%",
        "targets": [99, 95, 90],
        "type": "gauge"
      },
      {
        "id": 5,
        "title": "API Uptime (SLO)",
        "query": "up{job='api'}",
        "unit": "%",
        "targets": [99.9, 99, 95],
        "type": "stat"
      },
      {
        "id": 6,
        "title": "Latency Distribution",
        "query": "histogram_quantile([0.50, 0.75, 0.95, 0.99], rate(http_request_duration_ms_bucket[5m]))",
        "type": "table"
      }
    ]
  }
}
```

---

## 3. TRACKING DE PROGRESSO SEMANAL

### 3.1 Template de Status Semanal

```markdown
# Status Report - Week N

**Week of**: YYYY-MM-DD  
**Phase**: X/6  
**Overall Completion**: YY%

## Achievements This Week ✅

- [ ] Task 1: X hours invested
  - Result: Latency 2500ms → 300ms
  - Tests: 50 passing
  - Review: Merged to develop

- [ ] Task 2: X hours invested
  - Result: Added error handling types
  - Tests: 30 passing
  - Review: Waiting for approval

## In Progress ⏳

- Task A: XX% complete
  - Blocker: None
  - ETA: [Day/Date]
  
- Task B: XX% complete
  - Blocker: Waiting for review from Dev2
  - ETA: [Day/Date]

## Next Week 🚀

- [ ] Complete Task C
- [ ] Start Phase 3
- [ ] Performance benchmarks
- [ ] Team sync meeting

## Metrics Update 📊

| Métrica | Baseline | Current | Target | Status |
|---------|----------|---------|--------|--------|
| Latência (95p) | 2500ms | 350ms | <250ms | 🟡 Close |
| Test Coverage | 0% | 65% | 80% | 🟡 Close |
| Query Count | 101 | 1 | 1 | ✅ Met |
| Ingestion Success | 70% | 95% | 99% | 🟡 Good |

## Risks & Mitigations 🚨

| Risk | Severity | Mitigation | Status |
|------|----------|-----------|--------|
| Phase 1 query regression | High | Extensive testing, shadow reads | On track |
| Backward compatibility | Medium | API versioning | Planned |

## Approvals & Sign-offs 👥

- [ ] Tech Lead Review: _______  Date: ___
- [ ] Product Owner OK: _______  Date: ___

## Notes

- Team is on track for Phase 1 completion by Friday
- No blockers or escalations needed
- Ready to start Phase 2 parallel work next week
```

---

### 3.2 Weekly Metrics Collection Script

```bash
#!/bin/bash
# collect_metrics.sh - Collect baseline metrics weekly

echo "🔍 Collecting ParametroDosPostos Metrics - $(date)"

# Metric 1: Query Latency (via benchmark)
echo "1️⃣ Query Latency Benchmark..."
cargo bench --bin core_db --quiet 2>/dev/null | grep "get_postos" | tee -a metrics/latency.txt

# Metric 2: Test Coverage
echo "2️⃣ Test Coverage..."
cargo tarpaulin --out Xml --output-dir metrics 2>/dev/null | \
  grep -oP 'lines-valid="\K[^"]*' | tee -a metrics/coverage.txt

# Metric 3: Code Duplication (simple grep count)
echo "3️⃣ Code Duplication Scan..."
{
  echo "parse_csv occurrences: $(grep -r 'parse_csv' src/ --include='*.rs' | wc -l)"
  echo "insert_batch occurrences: $(grep -r 'insert_batch' src/ --include='*.rs' | wc -l)"
} | tee -a metrics/duplication.txt

# Metric 4: Cargo Clippy Warnings
echo "4️⃣ Cargo Clippy..."
cargo clippy --all 2>&1 | grep "warning:" | wc -l | tee -a metrics/warnings.txt

# Metric 5: Build Time
echo "5️⃣ Build Time..."
time cargo build --release 2>&1 | grep "real" | tee -a metrics/build_time.txt

# Generate report
echo ""
echo "📊 Metrics Summary:"
echo "===================="
echo "Last 5 collections:"
tail -5 metrics/latency.txt
tail -5 metrics/coverage.txt
tail -5 metrics/warnings.txt
```

---

## 4. MATRIZ DE DECISÃO GO/NO-GO

### 4.1 Decision Gate por Fase

Antes de cada fase começar, executar checklist:

#### Gate 0: Before PREP Phase

```
☐ Team aligned on plan
☐ Resource planning confirmed
☐ Git repository ready
☐ CI/CD infrastructure basic setup
☐ No production incidents open

→ DECISION: GO / HOLD / CANCEL
```

#### Gate 1: Before CORE_DB Phase (HIGH RISK)

```
TECHNICAL
☐ All Phase 0 tasks 100% complete
☐ Benchmark infrastructure working
☐ Feature flags implemented
☐ Query optimizer team assigned
☐ No active production incidents

OPERATIONAL  
☐ Staging environment ready for 24h validation
☐ Rollback procedure documented & tested
☐ On-call rotation established
☐ Customer communication plan ready
☐ Backup strategy verified

BUSINESS
☐ Product owner signoff
☐ No marketing/major event same week
☐ Team mental health: not overloaded

→ DECISION: GO / CONDITIONAL / HOLD / CANCEL

Conditional requirements:
- If YES: Additional 48h testing before deployment
- If YES: 2x code reviews instead of 1
```

#### Gate 2: Before Phases 2-3 (Parallel Start)

```
☐ Phase 1 (Core_DB) merged to develop
☐ Phase 1 metrics show improvement
☐ Staging validation 48h passed
☐ Team capacity for parallel work confirmed
☐ Communication plan for parallel development

→ DECISION: GO / HOLD
```

#### Gate 3: Before Phase 4-5 (Parallel Frontend/DevOps)

```
☐ Phases 1-3 mostly complete (80%+)
☐ API stable for 1 week
☐ No critical bugs reported
☐ Team ready for final push

→ DECISION: GO / HOLD
```

#### Gate 6: Before HARDENING Phase (Pre-Production)

```
☐ All phases merged to main branch
☐ All tests passing
☐ Performance benchmarks met
☐ Security audit completed
☐ Documentation complete
☐ Team trained

→ DECISION: GO / HOLD / ROLLBACK
```

---

### 4.2 Scoring Decision

**Use this scoring to make objective decisions**:

```
Scoring Rubric (0-100):

Quality (25 points max):
  20+ : All tests pass, coverage 80%+, zero clippy warnings
  15  : All tests pass, coverage 70%+, <5 warnings
  10  : Most tests pass, coverage 60%+, <10 warnings
  5   : Some tests fail, coverage <60%
  0   : No tests or major failures

Performance (25 points max):
  20+ : All benchmarks met, 95% latency target
  15  : Benchmarks met, 90% latency target
  10  : Benchmarks partial, 75% latency target
  5   : Benchmarks not met, <50% improvement
  0   : Performance regression

Stability (25 points max):
  20+ : 24h+ staging no incidents
  15  : 12h+ staging no incidents
  10  : 6h+ staging no incidents
  5   : <6h staging, minor issues
  0   : Production issues or blocking bugs

Business (25 points max):
  20+ : All stakeholders signed, no conflicts
  15  : Product owner signed, team ready
  10  : Mostly aligned, 1 concern
  5   : Misaligned on timing
  0   : Not ready, major concerns

TOTAL SCORE:
≥ 80 points → ✅ GO
60-79 points → ⚠️ CONDITIONAL (fix issues)
< 60 points → 🔴 HOLD (not ready)
```

---

## 5. INCIDENT RESPONSE PROCEDURES

### 5.1 Rollback Procedure por Fase

#### If Phase 1 (Core_DB) Fails

```
IMMEDIATE (< 1 min):
1. Feature flag: optimize_queries_v2 = false
2. Clear connection pool: pool.close()
3. Verify: curl http://localhost:3000/api/postos → 200 OK
4. Monitor: Check Prometheus for latency drop

IF FEATURE FLAG DOESN'T WORK (< 5 min):
5. Revert git commit: git revert <commit-hash>
6. Rebuild: cargo build --release
7. Redeploy

IF DATABASE CORRUPTION SUSPECTED (< 15 min):
8. Stop ingestions: kill -9 <ingest_pid>
9. Point-in-time restore: pg_restore from backup
10. Verify data integrity

COMMUNICATION (Ongoing):
- Alert on-call team
- Update status page
- Send customer notification
```

#### If Phase 2 (API) Fails

```
IMMEDIATE:
1. Revert API server: Rollback deployment
2. Route traffic to previous version
3. Verify: Tests against old version passing

IF BACKWARD INCOMPATIBILITY:
4. Keep both versions running
5. New clients use /api/v2, old use /api/v1
6. Schedule deprecation window
```

#### If Phase 3 (Ingestion) Fails

```
IMMEDIATE:
1. Stop all ingestions: TRUNCATE interdicoes_anp with checkpoint
2. Check data consistency: SELECT * FROM interdicoes_anp WHERE updated_at > NOW() - interval '1 hour'
3. If corrupted: ROLLBACK to backup

RE-RUN:
4. Re-run ingest with previous version
5. Monitor success rate (should be >99%)
6. Alert SRE if <95%
```

---

### 5.2 Escalation Path

```
Issue Detected
  ↓
  ├─ Severity: LOW
  │  └─ @on-call dev: Log ticket, address in next sprint
  │
  ├─ Severity: MEDIUM
  │  ├─ @on-call dev: Investigate & attempt fix
  │  └─ If not resolved in 15 min → @tech-lead
  │
  ├─ Severity: HIGH
  │  ├─ @on-call dev: Immediate triage
  │  ├─ @tech-lead: Immediately
  │  └─ Execute rollback procedure
  │
  └─ Severity: CRITICAL
     ├─ @on-call dev: Execute rollback NOW
     ├─ @tech-lead: Conference call
     ├─ @product: Customer notification
     └─ @cto: Executive update (if >30 min downtime)
```

---

## 6. SIGN-OFF TEMPLATES

### 6.1 Phase Completion Sign-Off

```markdown
# Phase X Completion Sign-Off

**Phase**: X - [Name]  
**Completed by**: [Dev Name]  
**Date**: YYYY-MM-DD  
**Duration**: X weeks  

## Deliverables Checklist

- [ ] All tasks in phase completed
- [ ] Tests passing: X/X
- [ ] Coverage: XX%
- [ ] Code reviewed: 2 approvals
- [ ] Merged to develop branch
- [ ] Staging validation: 48h+ passed
- [ ] Metrics baseline established

## Quality Gates

| Gate | Target | Actual | ✅/❌ |
|------|--------|--------|-------|
| Test Coverage | 80%+ | XX% | ✅ |
| Build Time | <45s | XXs | ✅ |
| Lint Warnings | 0 | X | ⚠️ |
| Documentation | 100% | XX% | ✅ |

## Known Issues

- [ ] None OR
- [ ] Issue #XX: [Description] (Planned for phase Y)

## Approvals

- [ ] Dev Lead: ________________  Date: ______
- [ ] Tech Lead: ________________  Date: ______
- [ ] Product Owner: ________________  Date: ______

**Status**: ✅ APPROVED - Ready for next phase
```

---

## 7. QUICK REFERENCE CHECKLIST

```
WEEKLY CHECKLIST:
☐ Collect metrics (use script above)
☐ Update status report template
☐ Run Phase gates if applicable
☐ Review risks and mitigations
☐ Get stakeholder sign-off

PHASE COMPLETION CHECKLIST:
☐ All tasks 100% complete
☐ Tests passing + coverage >80%
☐ Code reviewed + approved
☐ Staging validation 48h+
☐ Metrics show improvement
☐ Rollback plan documented
☐ Team sign-off obtained
☐ Next phase ready to start

PRE-PRODUCTION CHECKLIST:
☐ Load test passed
☐ Security audit passed
☐ Backup/restore verified
☐ Team trained
☐ Runbooks published
☐ On-call ready
☐ Customer communication sent
```

---

**Documento preparado**: 2026-05-01  
**Status**: ✅ Complete  
**Atualizar**: Semanalmente durante refactoring
