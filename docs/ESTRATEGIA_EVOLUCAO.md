# 🚀 ESTRATÉGIA DE EVOLUÇÃO - ParametroDosPostos
## De "Validador de Planilhas" para "Radar Nacional de Conformidade"

**Data**: 2026-05-01  
**Status**: Análise estratégica preliminar  
**Horizonte**: 3-6 meses (iterativo)

---

## 📊 ANÁLISE DA PROVOCAÇÃO

Sua análise está **100% correta**. Vou confirmar cada ponto:

### ✅ Diagnóstico Atual

| Aspecto | Status | Impacto |
|---------|--------|---------|
| **Fontes de dados** | 2 bases oficiais | ❌ Incompleto (estima-se 30-40% postos não cobertos) |
| **Lógica de validação** | Binary (regular/irregular) | ❌ Perde nuances e padrões |
| **Detecção de anomalias** | Nenhuma | ❌ Reativo apenas |
| **Geolocalização** | Não existe | ❌ Perde oportunidade de matching |
| **Automatização** | Manual | ❌ Não escalável |
| **Diferencial competitivo** | Nenhum | ❌ Qualquer um faz em 1 dia |

---

# 🎯 PRIORIZAÇÃO ESTRATÉGICA

## Roadmap de Impacto: O que fazer em qual ordem

### FASE 1 (4-6 semanas) - "Data Moat" 🏗️
**Objetivo**: Criar vantagem de dados incomparável  
**Por quê primeiro?**: Sem dados, nada funciona. Isso é a fundação.

#### 1.1 Integração Google Maps (CRÍTICO - MÁXIMO IMPACTO)

**Por quê?**
- Cobertura: ~98% dos postos brasileiros
- Atualização: em tempo real (usuários reportam)
- Dados: nome, endereço, coordenadas, avaliações
- Legalmente: dados públicos, API autorizada

**Decisões críticas**:
1. **Licenciamento**: Usar Google Places API com quotas grátis (limite: 25 requests/dia)
   - ❓ Decisão: Você vai rodar isso localmente ou em servidor?
   - → Isso muda o custo e a estratégia de scraping

2. **Estratégia de coleta**:
   - Opção A: Buscar por bbox (região × região)
   - Opção B: Usar nomes conhecidos como seed (ABCDiesel, Rede X)
   - Opção C: Combinar ambas
   - → **Recomendação**: Opção C (híbrida)

3. **Frequência de atualização**:
   - Primeira coleta: 1x
   - Depois: Semanal ou mensal?
   - → **Recomendação**: Mensal (dados não mudam tanto, mas cobrem novidades)

**MVP Técnico**:
```python
# Pseudocódigo
for regiao in regioes_brasil:
    resultados_maps = google_places_api.search(
        "posto de gasolina",
        location=regiao.centroid,
        radius=50km
    )
    # Desduplicar com base em coordenadas
    # Crosreferenciar com ANP
    if not_in_anp(resultado):
        flag_como("não_registrado")
```

**Desafios**:
- Google Maps tem postos fechados, desatualizados
- Precisão de coordenadas varia
- API tem limites (precisa gestão de quotas)

**Mitigação**:
- Cruzar com OpenStreetMap para validar
- Usar "reviews recentes" como proxy de atividade
- Implementar cache local

---

#### 1.2 Integração OpenStreetMap (SUPORTE)

**Por quê?**
- Dados abertos, sem limites de API
- Comunidade atualiza constantemente
- Estrutura conhecida (amenity=fuel)
- Legal sem restrições

**Decisão crítica**:
- Usar Overpass API ou baixar dump local?
- → **Recomendação**: Overpass API (mais simples, sempre atualizado)

**MVP**:
```python
# Overpass QL query
[bbox:${sul},${oeste},${norte},${leste}]
node["amenity"="fuel"];
out geom;
```

**Valor agregado**:
- Validar dados do Google Maps
- Preencher lacunas geográficas
- Dados de operadoras (tag "brand")

---

#### 1.3 Reconciliação de Dados (CORE TÉCNICO)

**O desafio real**: Mesmo posto pode ter 3 nomes diferentes

Exemplo:
```
Google Maps: "BR Distribuidora - Posto Salgado"
OpenStreetMap: "Salgado - Combustíveis"
ANP: "SALGADO COMERCIO DE COMBUSTIVEIS LTDA"
```

**Algoritmo de matching**:
```python
def match_posts(google_post, osm_post, anp_post):
    scores = []
    
    # 1. Distância geográfica (peso: 40%)
    dist_score = 1 - (distance / 100m)  # Se <100m, alta confiança
    
    # 2. Similaridade de nome (Levenshtein, peso: 35%)
    name_score = levenshtein_similarity(
        normalize(google_post.name),
        normalize(osm_post.name)
    )
    
    # 3. Endereço (peso: 15%)
    address_score = compare_addresses(...)
    
    # 4. Brand matching (peso: 10%)
    brand_score = match_brand(...)
    
    final_score = (
        dist_score * 0.40 +
        name_score * 0.35 +
        address_score * 0.15 +
        brand_score * 0.10
    )
    
    return final_score >= 0.75  # Threshold de confiança
```

**Desafios**:
- False positives (dois postos legítimos próximos)
- Nomes muito diferentes
- Endereços incompletos

**Mitigação**:
- Sempre deixar flag manual para verificação
- Usar geolocalização como tie-breaker
- Histórico de validações para treinar modelo

---

### FASE 2 (3-4 semanas) - "Risk Scoring" 🎯
**Objetivo**: Transformar validação binária em scoring 0-100  
**Pré-requisito**: Fase 1 completa

#### 2.1 Design do Score

```
RISK_SCORE = (
    weight_pmqc * dimension_pmqc +
    weight_anp * dimension_anp +
    weight_preco * dimension_preco +
    weight_avaliacao * dimension_avaliacao +
    weight_registro * dimension_registro
)

onde cada dimension = [0-100]
```

**Dimensões e Pesos (sugestão inicial)**:

| Dimensão | Peso | Critério | Score |
|----------|------|----------|-------|
| **PMQC** | 30% | Ausência → Presente | 0 → 100 |
| **ANP** | 25% | Dados inconsistentes | 20 (se falta) → 100 (tudo ok) |
| **Preço** | 20% | Desvio regional | σ < 1.5 → 100 (normal) |
| **Avaliação** | 15% | Média Google Maps | <3.5★ → 100 (5★) |
| **Registro** | 10% | CNPJ/Cadastro | Inativo → 0, Ativo → 100 |

**Interpretação**:
- 0-30: 🔴 Alto Risco (investigar imediatamente)
- 31-70: 🟡 Atenção (monitorar)
- 71-100: 🟢 Regular (confiar)

**Exemplo prático**:
```
Posto: "Gasolina Plus" - Rua X, SP

PMQC: Ausente [-50 pontos] → Score: 20
ANP: Cadastro ativo [+20] → Score: 80
Preço: 20% abaixo média regional [-40] → Score: 30
Avaliação: 2.2 estrelas (n=12) [-50] → Score: 40
Registro: CNPJ inativo [-100] → Score: 0

SCORE FINAL = (0.30×20 + 0.25×80 + 0.20×30 + 0.15×40 + 0.10×0)
            = 6 + 20 + 6 + 6 + 0
            = 38 → 🟡 ATENÇÃO
```

**Decisões críticas**:
1. Pesos iniciais: usar benchmarks de órgãos públicos ou iterar?
   - → **Recomendação**: Iterar com dados reais (feedback loop)

2. Como lidar com dados faltantes?
   - → **Recomendação**: Defaultar para "incerteza" (score 50), não punir

3. Temporal: score muda com o tempo?
   - → **Recomendação**: Sim, histórico de scores para detectar deterioração

---

#### 2.2 Detecção de Anomalias

Além do score, adicionar flags de alerta:

```python
FLAGS = [
    "ausente_pmqc",           # Nunca vistoriado
    "preco_outlier",          # >2σ desvio
    "avaliacao_negativa_recente",  # <3★ em últimos 30 dias
    "cnpj_cancelado",         # Status de cadastro
    "endereco_mismatch",      # Coordenadas não batem
    "duplicata_suspeita",     # 2+ postos mesma localização
    "nao_encontrado_maps",    # Esperado em Maps, não achou
    "inativo_social_media",   # Sem atualização há 6 meses
]
```

---

### FASE 3 (2-3 semanas) - "Geolocalização & Mapping" 🗺️
**Objetivo**: Dashboard geoespacial  
**Pré-requisito**: Fase 1-2

#### 3.1 PostGIS Setup

Adicionar suporte geoespacial:
```sql
-- Criar coluna de geometria
ALTER TABLE postos ADD COLUMN geom GEOMETRY(Point, 4326);

-- Criar índice GiST para performance
CREATE INDEX idx_postos_geom ON postos USING GIST(geom);

-- Query de proximidade: "Postos a 5km de aqui?"
SELECT * FROM postos
WHERE ST_DWithin(geom, ST_Point(-46.633309, -23.550520), 5000)
  AND risk_score > 50;
```

#### 3.2 Detecção de Duplicatas

```sql
-- Encontrar postos muito próximos com nomes diferentes
SELECT 
    p1.id, p1.nome, p1.risk_score,
    p2.id, p2.nome, p2.risk_score,
    ST_Distance(p1.geom, p2.geom) as distancia
FROM postos p1
JOIN postos p2 ON ST_DWithin(p1.geom, p2.geom, 100)  -- Mesma localização
WHERE p1.id < p2.id
  AND levenshtein_similarity(p1.nome, p2.nome) < 0.7  -- Nomes diferentes
ORDER BY distancia;
```

#### 3.3 Análise Regional

Identificar vazios de fiscalização:
```sql
-- Regiões com baixo score médio
SELECT 
    municipio,
    COUNT(*) as total_postos,
    AVG(risk_score) as score_medio,
    MIN(risk_score) as pior_score,
    COUNT(CASE WHEN risk_score < 30 THEN 1 END) as alto_risco_count
FROM postos
GROUP BY municipio
HAVING AVG(risk_score) < 50
ORDER BY score_medio;
```

---

### FASE 4 (2 semanas) - "Automatização & Orquestração" ⚙️
**Objetivo**: Processamento diário sem intervenção  
**Pré-requisito**: Fases 1-3

#### 4.1 Pipeline ETL

```
[Google Maps API] ─┐
[OpenStreetMap]  ──┼─→ [Data Harmonization] ─→ [Matching] ─→ [Scoring] ─→ [DB]
[ANP Dump]       ──┤                                                           ↓
[PMQC CSV]       ──┤                                                     [Alerts]
[Receita Federal]─┘
                                                                             ↓
                                                                      [Dashboard]
```

#### 4.2 Agendamento

Sugestão com Apache Airflow:
```python
from airflow import DAG
from datetime import timedelta

default_args = {
    'retries': 2,
    'retry_delay': timedelta(hours=1),
}

dag = DAG(
    'postos_daily_update',
    default_args=default_args,
    schedule_interval='0 2 * * *',  # 2 AM todo dia
)

# Task 1: Fetch Google Maps
fetch_maps = PythonOperator(task_id='fetch_maps', ...)

# Task 2: Fetch OpenStreetMap
fetch_osm = PythonOperator(task_id='fetch_osm', ...)

# Task 3: Harmonize
harmonize = PythonOperator(task_id='harmonize', ...)

# Task 4: Match & Score
match_score = PythonOperator(task_id='match_score', ...)

# Task 5: Generate Alerts
alerts = PythonOperator(task_id='generate_alerts', ...)

# Task 6: Send Notifications
notify = EmailOperator(task_id='notify', ...)

# Dependencies
[fetch_maps, fetch_osm] >> harmonize >> match_score >> alerts >> notify
```

#### 4.3 Alertas Inteligentes

Apenas alertas que agregam valor:
```python
ALERTS = {
    "novo_posto_detectado": {
        "condicao": "em_maps OR osm but NOT em_anp",
        "severidade": "info",
        "freq": "daily_digest",
    },
    "posto_deteriorado": {
        "condicao": "risk_score_today < risk_score_yesterday - 15",
        "severidade": "high",
        "freq": "immediate",
    },
    "preco_anomalia": {
        "condicao": "preco desvio > 2.5σ",
        "severidade": "medium",
        "freq": "daily",
    },
    "cluster_risco": {
        "condicao": "5+ postos alto_risco em raio 10km",
        "severidade": "high",
        "freq": "weekly",
    },
}
```

---

### FASE 5 (3 semanas) - "Camada Regulatória" 🏛️
**Objetivo**: Validações que interessam a autoridades  
**Pré-requisito**: Fases 1-4

#### 5.1 Conformidade CNPJ

```sql
-- Integrar Receita Federal (dados públicos)
SELECT 
    p.id, p.cnpj, rf.razao_social, rf.situacao_cadastral,
    CASE 
        WHEN rf.situacao_cadastral = 'Ativa' THEN 100
        WHEN rf.situacao_cadastral = 'Suspensa' THEN 30
        ELSE 0
    END as compliance_score
FROM postos p
LEFT JOIN receita_federal rf ON p.cnpj = rf.cnpj;
```

#### 5.2 Conformidade de Combustível

```sql
-- Validar especificações (etanol, volatilidade, etc)
SELECT 
    p.id, p.nome,
    CASE 
        WHEN pmqc.etanol BETWEEN 18 AND 22 THEN 100
        WHEN pmqc.etanol BETWEEN 15 AND 25 THEN 50
        ELSE 0
    END as etanol_compliance,
    pmqc.ultima_vistoria
FROM postos p
LEFT JOIN pmqc_resultados pmqc ON p.id = pmqc.posto_id
WHERE pmqc.ultima_vistoria < DATE_SUB(NOW(), INTERVAL 180 DAY)
  AND pmqc.ultima_vistoria IS NOT NULL;
```

#### 5.3 Frequência de Fiscalização

```sql
-- Flag postos que não foram vistoriados há muito tempo
SELECT 
    p.id, p.nome,
    DATEDIFF(NOW(), pmqc.ultima_vistoria) as dias_sem_vistoria,
    CASE 
        WHEN dias_sem_vistoria > 365 THEN 'Crítico'
        WHEN dias_sem_vistoria > 180 THEN 'Overdue'
        ELSE 'Ok'
    END as status_vistoria
FROM postos p
LEFT JOIN pmqc_resultados pmqc ON p.id = pmqc.posto_id;
```

---

# 🏗️ ARQUITETURA RECOMENDADA

## Stack Sugerido (Iterativo)

### MVP (Fase 1-2)
```
┌─────────────────────────────────────┐
│      Frontend (React)               │
│  ├─ Busca simples                   │
│  ├─ Mapa básico                     │
│  └─ Lista com scores                │
└──────────────┬──────────────────────┘
               │ HTTP REST
┌──────────────▼──────────────────────┐
│      Backend (Python FastAPI)       │
│  ├─ /api/postos                     │
│  ├─ /api/risk-score/{id}            │
│  ├─ /api/search                     │
│  └─ /api/geo (bbox search)          │
└──────────────┬──────────────────────┘
               │ SQLAlchemy ORM
┌──────────────▼──────────────────────┐
│   Database (PostgreSQL + PostGIS)   │
│  ├─ Tabela: postos                  │
│  ├─ Tabela: risco_historico         │
│  ├─ Tabela: alertas                 │
│  └─ Índices: geom, risk_score       │
└─────────────────────────────────────┘
```

### Com Automação (Fase 4)
```
Adicionar:
  ├─ Apache Airflow (orquestração)
  ├─ Redis (cache + queue)
  ├─ Celery (tasks assíncronas)
  └─ Logs estruturados (ELK ou similar)
```

### Com API Pública (Fase 5+)
```
Adicionar:
  ├─ API públic (limites, autenticação)
  ├─ Cache layer (CloudFlare)
  └─ Monitoring (DataDog, Prometheus)
```

---

## Por Que Python (não Rust)?

Seu projeto atual é Rust, mas sugiro Python para esta evolução:

| Aspecto | Rust | Python |
|---------|------|--------|
| **Data processing** | Médio | ⭐⭐⭐ |
| **ML/Scoring** | Difícil | ⭐⭐⭐ |
| **API development** | Bom | ⭐⭐⭐ |
| **Prototipagem** | Lento | ⭐⭐⭐ |
| **Integração APIs** | Médio | ⭐⭐⭐ |
| **Community** | Pequeno | ⭐⭐⭐ |

**Decisão**: Rust para performance crítica + Python para lógica de dados
- Rust: API core (se 10k+ RPS no futuro)
- Python: ETL + Scoring (onde estará 80% do valor)

---

# 🎯 PRIORIZAÇÃO FINAL: Roadmap 3-6 Meses

## Sprint 0 (Hoje - 1 semana)
```
[ ] Definir 3 decisões críticas (abaixo)
[ ] Setup infraestrutura (DB, Airflow)
[ ] Estudar Overpass API + Google Places API
[ ] Prototipar matching simples (Levenshtein)
```

## Sprint 1-2 (Semana 2-3)
```
[ ] Google Places API integration (MVP)
[ ] OpenStreetMap Overpass integration
[ ] Reconciliação básica (georeferência)
[ ] 1ª versão de dados harmonizados
```

## Sprint 3-4 (Semana 4-5)
```
[ ] Risk scoring framework (V1)
[ ] Dashboard básico (React + MapBox)
[ ] Testes com 100k+ postos
[ ] Validação com órgãos públicos (feedback)
```

## Sprint 5-6 (Semana 6-8)
```
[ ] Airflow pipeline (diário)
[ ] Alertas + Email notifications
[ ] Camada regulatória (CNPJ, PMQC)
[ ] Performance optimization
```

## Sprint 7+ (Futuro)
```
[ ] API pública + docs
[ ] App mobile (React Native)
[ ] ML para previsão de irregularidades
[ ] Monetização (se faz sentido)
```

---

# 🚨 DECISÕES CRÍTICAS (AGORA)

## Decisão 1: Escala & Infraestrutura

**A Pergunta**: Isso vai rodar onde e para quem?

**Opções**:
1. **Local (seu laptop)** → Data hobby project
2. **Servidor + URL pública** → Para compartilhar com amigos
3. **Escala pública** → Consumidores, reguladores, etc.

**Recomendação**: 
- **Opção 2 inicialmente** (servidor barato, público, validar interesse)
- Depois escalar se houver demanda real

**Implicação**:
- Opção 2 → Precisará de CI/CD, monitoring, backup
- Custo: ~R$50-100/mês (DB + Server + APIs)

---

## Decisão 2: Modelo de Dados Único ou Federado

**A Pergunta**: Um único "postos" master ou múltiplas tabelas por fonte?

**Opções**:
1. **Unified model**: Tudo em "postos" com campos de origem
2. **Federated**: Tabelas separadas (postos_anp, postos_maps, postos_osm)

**Recomendação**: **Opção 1 (unified)**
- Mais fácil de raciocinar
- Queries simples
- Matching centralizador

**Schema proposto**:
```sql
CREATE TABLE postos (
    id UUID PRIMARY KEY,
    
    -- Dados consolidados (master)
    nome_normalizado VARCHAR,
    cnpj VARCHAR,
    endereco_normalizado VARCHAR,
    geom GEOMETRY(Point, 4326),
    
    -- Scores & Status
    risk_score INT (0-100),
    last_score_update TIMESTAMP,
    
    -- Origem dos dados
    em_anp BOOLEAN DEFAULT FALSE,
    em_pmqc BOOLEAN DEFAULT FALSE,
    em_maps BOOLEAN DEFAULT FALSE,
    em_osm BOOLEAN DEFAULT FALSE,
    
    -- Referências cruzadas
    anp_id VARCHAR,
    maps_id VARCHAR,
    osm_id VARCHAR,
    
    -- Metadata
    data_criacao TIMESTAMP,
    data_ultima_atualizacao TIMESTAMP,
    confianca_match FLOAT (0-1),  -- Quão confiável é o matching
);
```

---

## Decisão 3: Algoritmo de Scoring - Abordagem

**A Pergunta**: Scoring determinístico ou com ML?

**Opções**:
1. **Determinístico** (regras fixas): 
   - Mais simples
   - Previsível
   - Menos preciso

2. **ML-based** (XGBoost, logistic regression):
   - Mais preciso
   - Aprende com o tempo
   - Complexo de manter

3. **Híbrido** (regras + ML):
   - Melhor dos dois mundos
   - Mais complexo

**Recomendação**: **Opção 1 (determinístico V1) → Híbrido depois**
- MVP com regras claras e documentadas
- Depois, quando tiver dados históricos, treinar modelo
- Isso permite validação regulatória (explicabilidade)

---

# 💰 Questão de Monetização & Impacto

## Onde está o real valor?

### Opção A: B2B (Órgãos Públicos)
**Clientes**: ANP, INMETRO, Vigilância Sanitária  
**Value Prop**: "Detecte postos não registrados automaticamente"  
**Modelo**: SaaS customizado  
**Viabilidade**: 🟡 Médio (vende lento, mas ticket alto)

### Opção B: B2C (Consumidores)
**Clientes**: Motoristas, apps de viagem  
**Value Prop**: "Score de confiabilidade do posto" (tipo Uber rating)  
**Modelo**: API pública + App  
**Viabilidade**: 🟢 Alto (muitas pessoas buscam isso)

### Opção C: B2G (Governo)
**Clientes**: Prefeituras, Defesa do Consumidor  
**Value Prop**: "Dashboard de compliance regional"  
**Modelo**: Governo contrata análise  
**Viabilidade**: 🟡 Médio (burocracia, mas volume)

### Opção D: Open Source / Data
**Foco**: Liberar dataset + API pública  
**Value Prop**: "Infraestrutura pública, bem comum"  
**Modelo**: Sem lucro direto (reputação, doações)  
**Viabilidade**: 🟢 Alto (comunidade, impact)

**Recomendação**: 
1. **MVP**: Foco em **Opção B + D** (máximo impacto, viável para hobby project)
2. **Futuro**: Explorar B2B se houver tração

---

# ✅ RESUMO: Próximos Passos Imediatos

### Semana 1
- [ ] Responder as 3 decisões críticas (acima)
- [ ] Setup: PostgreSQL local + PostGIS
- [ ] Python project scaffold (FastAPI)
- [ ] Estudar Google Places API + Overpass API

### Semana 2
- [ ] Prototipar Google Places fetch (1 estado)
- [ ] Prototipar Overpass fetch (1 estado)
- [ ] Criar harmonização básica
- [ ] Visualizar dados (primeiros 1k postos)

### Semana 3
- [ ] Matching algorithm V1
- [ ] Risk score framework V1
- [ ] Dashboard React básico (map view)

### Semana 4+
- [ ] Validar com dados reais
- [ ] Feedback regulatório (conversa com órgãos)
- [ ] Escalar para todo Brasil
- [ ] Airflow pipeline

---

## Conclusão

Sua visão de transformar isso em um **"Radar Nacional"** é **absolutamente viável**. Os maiores gargalos são:

1. **Dados de fontes adicionais** (Google Maps é ouro)
2. **Matching robusto** (mesmo posto, nomes diferentes)
3. **Scoring inteligente** (que capture o que realmente importa)
4. **Automação** (para não ser manual)

O resto é implementação. A chance de impacto é **real** — consumidores, empresas e governo precisam disso.

**Próximo passo: Qual das 3 decisões críticas você quer decidir primeiro?**
