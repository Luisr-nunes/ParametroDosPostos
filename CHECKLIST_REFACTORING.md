# ✅ REFACTORING CHECKLIST
## ParametroDosPostos - Quick Reference

**Print this and post on your desk**

---

## 🎯 OVERALL OBJECTIVES

- [ ] Team understands why we're doing this (70% latency reduction)
- [ ] Executive buy-in secured
- [ ] Budget approved (~$19.5k)
- [ ] Resources allocated (2-3 devs + 1 SRE)
- [ ] Timeline agreed (8-10 weeks)

---

## 📊 BASELINE METRICS (Before Any Work)

Measure these BEFORE Phase 0 starts:

- [ ] **Query Latency**: 2500ms (95p) - Check: `time curl api/postos`
- [ ] **Query Count**: 101 queries - Check logs: `RUST_LOG=debug`
- [ ] **Connection Pool**: 5/5 saturated - Check: `Connection pool metrics`
- [ ] **Test Coverage**: 0% - Check: `cargo tarpaulin`
- [ ] **Build Time**: ~60s - Check: `time cargo build`
- [ ] **Clippy Warnings**: ~20 - Check: `cargo clippy`
- [ ] **Ingestion Success**: ~70% - Check: Last 5 runs
- [ ] **API Uptime**: TBD - Start monitoring now

**→ Store these values in: `metrics/baseline_week0.txt`**

---

## PHASE 0: PREPARATION
**Duration**: Week 1-2 | **Owner**: Dev1 | **Effort**: 50h

### ✅ Completion Checklist

- [ ] **Task 0.1**: Cargo.toml editions fixed (2024 → 2021)
  - Verify: `cargo check --all` passes
  - Time: 30 min
  - Owner: Dev1

- [ ] **Task 0.2**: Logging infrastructure (tracing framework)
  - Verify: `RUST_LOG=debug cargo run` shows JSON logs
  - Time: 2h
  - Owner: Dev1

- [ ] **Task 0.3**: .env configuration file created
  - Verify: All config vars documented
  - Time: 1h
  - Owner: Dev1

- [ ] **Task 0.4**: Test infrastructure setup
  - Verify: `cargo test --all` finds tests
  - Time: 1.5h
  - Owner: Dev1 + Dev2

- [ ] **Task 0.5**: GitHub Actions CI/CD pipeline
  - Verify: PR triggers workflow, all checks pass
  - Time: 3h
  - Owner: Dev1

- [ ] **Task 0.6**: Health check endpoint
  - Verify: `curl /health` returns JSON
  - Time: 1h
  - Owner: Dev1

### 📋 Phase 0 Sign-Off

- [ ] Cargo check: ✅ all pass
- [ ] Cargo fmt: ✅ all pass
- [ ] Cargo clippy: ✅ zero warnings
- [ ] Cargo test: ✅ all pass
- [ ] CI/CD: ✅ workflow passing
- [ ] Documentation: ✅ complete
- [ ] Code review: ✅ 2 approvals
- [ ] Merged to develop: ✅ yes
- [ ] Team ready for Phase 1: ✅ yes

---

## PHASE 1: CORE_DB LAYER ⚠️ HIGH RISK
**Duration**: Week 2-4 | **Owner**: Dev1 | **Effort**: 80h

### ✅ Completion Checklist

- [ ] **Feature branch created**: `feature/phase-1-core-db`
- [ ] **Query optimization**: N+1 → JOINs implemented
  - Verify: Query count: 101 → 1
  - Time: 3h
  
- [ ] **Benchmarking done**: Before/after latency measured
  - Verify: 2500ms → <250ms (target met)
  - Time: 2h

- [ ] **Error types defined**: PostoError, DbError
  - Verify: All Results use typed errors
  - Time: 2h

- [ ] **Logging added**: All queries logged with timings
  - Verify: Debug logs show duration_ms
  - Time: 1.5h

- [ ] **Transaction support**: Batch operations
  - Verify: 1000+ records in single tx
  - Time: 2h

- [ ] **Tests written**: >80% coverage for core_db
  - Verify: `cargo tarpaulin` shows 80%+
  - Time: 3h

- [ ] **Feature flag**: Fallback to old queries
  - Verify: Can toggle ENABLE_OPTIMIZED_QUERIES
  - Time: 1h

### 🧪 Phase 1 Test Validation

```bash
# Test 1: Query count reduced
cargo test test_get_postos_has_all_relations -- --nocapture
# Expected: Query count = 1

# Test 2: Latency target met
cargo bench  # Should show 150ms vs 2500ms baseline

# Test 3: No regressions
curl http://staging/api/v1/postos
# Should have same data structure

# Test 4: Feature flag works
DATABASE_URL=... ENABLE_OPTIMIZED_QUERIES=false cargo run
# Should use legacy path
```

### ⚠️ Risk Gates (Must Pass Before Go-Live)

- [ ] Test coverage: >80% ✅
- [ ] Benchmark: 70%+ latency reduction ✅
- [ ] Shadow reads: Zero divergence ✅
- [ ] Staging validation: 48h+ passed ✅
- [ ] Code review: 2 approvals ✅

**Decision**: ✅ GO / ⚠️ CONDITIONAL / 🔴 HOLD

---

## PHASE 2: API LAYER
**Duration**: Week 4-6 (parallel) | **Owner**: Dev2 | **Effort**: 80h

### ✅ Completion Checklist

- [ ] **Request/Response types**: serde validation
  - Verify: Endpoints type-safe
  - Time: 2h

- [ ] **Error handling**: Structured 400/500 responses
  - Verify: Errors return proper JSON
  - Time: 1.5h

- [ ] **API versioning**: /api/v1, /api/v2
  - Verify: Both versions work
  - Time: 2h

- [ ] **OpenAPI spec**: Generated + Swagger UI
  - Verify: /api/docs shows spec
  - Time: 2h

- [ ] **Rate limiting**: 100 req/min per IP
  - Verify: 150 requests blocked at 100+
  - Time: 1.5h

- [ ] **Metrics**: Prometheus endpoint
  - Verify: /metrics returns data
  - Time: 2h

### 📊 Phase 2 Metrics

- [ ] **API Response Time**: <250ms (95p)
- [ ] **Error Rate**: <1% (4xx/5xx)
- [ ] **Documentation**: 100% endpoints covered

---

## PHASE 3: DATA INGESTION
**Duration**: Week 5-7 (parallel) | **Owner**: Dev1 | **Effort**: 90h

### ✅ Completion Checklist

- [ ] **CSV/JSON parsing**: Consolidated in core_db::parsers
  - Verify: Single parse_csv() function used everywhere
  - Time: 2.5h

- [ ] **Transactional batches**: 1000 records per tx
  - Verify: INSERT batches atomically
  - Time: 2h

- [ ] **Error tracking**: Success/fail counts logged
  - Verify: Metrics show 99%+ success
  - Time: 1.5h

- [ ] **Retry logic**: Exponential backoff (1s, 2s, 4s)
  - Verify: Retries work on network failure
  - Time: 1.5h

- [ ] **Idempotency**: ON CONFLICT DO UPDATE
  - Verify: 2x run = same count (no dups)
  - Time: 1h

- [ ] **Monitoring**: Dashboard shows ingestion health
  - Verify: Grafana dashboard alive
  - Time: 3h

### 📊 Phase 3 Metrics

- [ ] **Success Rate**: 99%+ (was 70%)
- [ ] **Error Visibility**: 100% logged
- [ ] **Duplicate Prevention**: 0 duplicates on retry

---

## PHASE 4: FRONTEND
**Duration**: Week 6-9 (parallel) | **Owner**: Dev2 | **Effort**: 100h

### ✅ Completion Checklist

- [ ] **Map component**: Extracted and reusable
- [ ] **Search component**: Isolated logic
- [ ] **React Router**: Navigation working
- [ ] **API client**: Type-safe queries
- [ ] **Error boundaries**: Error handling
- [ ] **Loading states**: UX feedback

### 📊 Phase 4 Metrics

- [ ] **Component count**: 1 monolithic → 6+ modular
- [ ] **App.tsx lines**: 500+ → <150

---

## PHASE 5: DEVOPS
**Duration**: Week 8-10 (parallel) | **Owner**: SRE | **Effort**: 70h

### ✅ Completion Checklist

- [ ] **.env management**: Config via environment
- [ ] **Feature flags**: Runtime toggles working
- [ ] **Docker health checks**: Services wait for dependencies
- [ ] **Prometheus setup**: Metrics collection working
- [ ] **Grafana**: Dashboard created
- [ ] **Deployment script**: Blue-green automation

### 📊 Phase 5 Metrics

- [ ] **Deployment time**: <2 min (was manual ~15 min)
- [ ] **MTTR**: <5 min (was ~30 min)

---

## PHASE 6: HARDENING
**Duration**: Week 9-10 | **Owner**: Full Team | **Effort**: 65h

### ✅ Completion Checklist

- [ ] **Load test**: 100 concurrent users passed
  - Verify: 95th percentile latency <500ms
  - Time: 2h

- [ ] **Security audit**: OWASP checked
  - Verify: Zero vulnerabilities found
  - Time: 3h

- [ ] **Backup/restore**: Tested and documented
  - Verify: Restore <10 min
  - Time: 2h

- [ ] **Team training**: All devs comfortable
  - Verify: Each dev can handle incident
  - Time: 2h

- [ ] **Documentation**: Architecture + runbooks
  - Verify: 100% complete
  - Time: 2h

- [ ] **Go-live checklist**: All items signed
  - Verify: No gaps
  - Time: 1h

### 📋 Go-Live Checklist

**Code Quality**:
- [ ] cargo check: ✅ pass
- [ ] cargo test: ✅ pass
- [ ] cargo clippy: ✅ zero warnings
- [ ] Coverage: ✅ >80%

**Performance**:
- [ ] Latency (95p): ✅ <250ms
- [ ] Query count: ✅ = 1
- [ ] Load test: ✅ 100 users @ <500ms

**Reliability**:
- [ ] Data success: ✅ 99%+
- [ ] Error visibility: ✅ 100%
- [ ] No duplicates: ✅ verified

**Operations**:
- [ ] Monitoring: ✅ live
- [ ] Alerting: ✅ configured
- [ ] Runbooks: ✅ written
- [ ] Team trained: ✅ yes

**Security**:
- [ ] OWASP audit: ✅ pass
- [ ] No SQL injection: ✅ verified
- [ ] No credentials in code: ✅ verified
- [ ] Rate limiting: ✅ enabled

**Deployment**:
- [ ] Feature flags: ✅ verified
- [ ] Rollback plan: ✅ documented
- [ ] Staging validation: ✅ 48h+ passed
- [ ] On-call: ✅ assigned

**Documentation**:
- [ ] Architecture: ✅ complete
- [ ] API docs: ✅ complete
- [ ] Runbooks: ✅ published
- [ ] Changelog: ✅ updated

**Approvals**:
- [ ] Tech Lead: ________________  Date: ___
- [ ] Product Owner: ________________  Date: ___
- [ ] Operations Lead: ________________  Date: ___

---

## 📈 WEEKLY TRACKING

### Week 1-2: PHASE 0
- [ ] Mon: Cargo.toml + logging
- [ ] Wed: .env + tests
- [ ] Fri: CI/CD + health endpoint
- [ ] Status: Green / Yellow / Red

### Week 2-4: PHASE 1
- [ ] Week start: Query optimization design
- [ ] Mid-week: Benchmarking baseline
- [ ] End-week: PR review + merge
- [ ] Status: Green / Yellow / Red

### Week 4-6: PHASE 2-3 PARALLEL
- [ ] API team: Versioning + OpenAPI
- [ ] Ingestion team: CSV consolidation + transactions
- [ ] Status: Green / Yellow / Red

### Week 6-9: PHASE 4-5 PARALLEL
- [ ] Frontend: Component extraction
- [ ] DevOps: Monitoring + automation
- [ ] Status: Green / Yellow / Red

### Week 9-10: PHASE 6
- [ ] Load testing
- [ ] Security audit
- [ ] Final validation
- [ ] Status: Green / Yellow / Red

---

## 🚨 INCIDENT RESPONSE

### If Phase 1 Fails (N+1 Queries Break)

**Immediate**:
1. [ ] Feature flag OFF: `ENABLE_OPTIMIZED_QUERIES=false`
2. [ ] Verify: `curl api/postos` returns 200
3. [ ] Monitor: Check Prometheus dashboard
4. [ ] Timeline: <1 minute

**If feature flag doesn't work**:
5. [ ] Revert commit: `git revert <hash>`
6. [ ] Rebuild: `cargo build --release`
7. [ ] Redeploy to staging
8. [ ] Timeline: <5 minutes

### If Phase 3 Fails (Data Ingestion Corrupted)

**Immediate**:
1. [ ] Stop ingestions: `kill -9 <pid>`
2. [ ] Check logs: `SELECT * FROM interdicoes_anp WHERE updated_at > NOW() - interval '1h'`
3. [ ] If corrupted: Restore from backup
4. [ ] Timeline: <15 minutes

### If Any Phase Blocks (Blocker)

1. [ ] Document issue in detail
2. [ ] Escalate to @tech-lead
3. [ ] Create task in Jira (urgent)
4. [ ] Continue work on non-blocked tasks

---

## 📞 ESCALATION

**Blocker?** → Slack @tech-lead immediately  
**Disagreement on approach?** → Schedule sync with @tech-lead  
**Need approval?** → Email @cto with context  
**Customer impact?** → Alert @product + @cto  

**Weekly Sync**: Tuesdays 10 AM  
**Channel**: #refactoring-phase-0  

---

## 📊 SUCCESS METRICS TARGETS

| Metric | Baseline | Target | Current | Status |
|--------|----------|--------|---------|--------|
| Query Latency (95p) | 2500ms | <250ms | TBD | ⏳ |
| Query Count | 101 | 1 | TBD | ⏳ |
| Build Time | 60s | <45s | TBD | ⏳ |
| Test Coverage | 0% | 80%+ | TBD | ⏳ |
| Clippy Warnings | 20 | 0 | TBD | ⏳ |
| Ingestion Success | 70% | 99% | TBD | ⏳ |
| API Uptime | N/A | 99.9% | TBD | ⏳ |

---

## 🎯 DECISION GATES

### Go/No-Go Before Each Phase

**Gate 0** (Before PREP):
- [ ] Team alignment
- [ ] Resources confirmed
- [ ] Decision: ✅ GO / ⚠️ HOLD

**Gate 1** (Before CORE_DB - HIGH RISK):
- [ ] Phase 0 complete ✅
- [ ] Benchmarking infrastructure ✅
- [ ] Feature flags tested ✅
- [ ] Decision: ✅ GO / ⚠️ CONDITIONAL / 🔴 HOLD

**Gate 2** (Before PHASES 2-3):
- [ ] Phase 1 merged ✅
- [ ] Metrics show improvement ✅
- [ ] Team capacity ✅
- [ ] Decision: ✅ GO / ⚠️ HOLD

**Gate 6** (Before GO-LIVE):
- [ ] All phases 100% ✅
- [ ] All tests passing ✅
- [ ] Security audit ✅
- [ ] Load test ✅
- [ ] Decision: ✅ GO / 🔴 HOLD / 🔴 ROLLBACK

---

## 🏁 FINAL CHECKLIST

Before declaring COMPLETE:

- [ ] All 6 phases finished
- [ ] All tests passing
- [ ] All metrics targets met
- [ ] No open issues tagged "refactoring"
- [ ] Documentation complete
- [ ] Team trained
- [ ] Incident procedures verified
- [ ] Customers notified
- [ ] Celebrate! 🎉

---

**Print Date**: _____________  
**Team**: _____________  
**Tech Lead**: _____________  
**Completed**: _____________  

---

*Last Update: 2026-05-01*  
*Next Update: Weekly during refactoring*  
