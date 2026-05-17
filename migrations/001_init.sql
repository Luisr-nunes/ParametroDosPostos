-- =============================================================================
-- Migration: 001_init.sql
-- Descrição: Schema inicial do banco de dados ParametroDosPostos
-- Requer: PostGIS extension
-- =============================================================================

CREATE EXTENSION IF NOT EXISTS postgis;

-- Tabela de Postos de Combustível
CREATE TABLE IF NOT EXISTS postos (
    id SERIAL PRIMARY KEY,
    cnpj VARCHAR(18) UNIQUE NOT NULL,
    razao_social VARCHAR(255) NOT NULL,
    nome_fantasia VARCHAR(255),
    endereco TEXT NOT NULL,
    bairro VARCHAR(100),
    municipio VARCHAR(100) NOT NULL,
    uf VARCHAR(2) NOT NULL,
    cep VARCHAR(10),
    bandeira VARCHAR(100),
    localizacao GEOMETRY(Point, 4326),
    status_autorizacao VARCHAR(50), -- ATIVO, INATIVO
    data_atualizacao TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tabela de Interdições (Consulta Postos Web)
CREATE TABLE IF NOT EXISTS interdicoes_anp (
    id SERIAL PRIMARY KEY,
    posto_cnpj VARCHAR(18) REFERENCES postos(cnpj),
    motivo TEXT NOT NULL,
    data_interdicao DATE NOT NULL,
    data_desinterdicao DATE,
    status VARCHAR(50) NOT NULL, -- INTERDITADO, DESINTERDITADO
    documento_referencia VARCHAR(100),
    data_registro TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tabela de Qualidade PMQC
CREATE TABLE IF NOT EXISTS inspecoes_pmqc (
    id SERIAL PRIMARY KEY,
    posto_cnpj VARCHAR(18) REFERENCES postos(cnpj),
    combustivel VARCHAR(50) NOT NULL, -- Gasolina Comum, Etanol, Diesel
    parametro VARCHAR(100) NOT NULL, -- Aspecto, Cor, Teor de Álcool, etc.
    resultado_obtido VARCHAR(100),
    limite_especificacao VARCHAR(100),
    conforme BOOLEAN NOT NULL,
    data_coleta DATE NOT NULL,
    boletim VARCHAR(100),
    data_registro TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Índices para performance
CREATE INDEX IF NOT EXISTS idx_postos_localizacao ON postos USING GIST (localizacao);
CREATE INDEX IF NOT EXISTS idx_postos_municipio ON postos (municipio);
CREATE INDEX IF NOT EXISTS idx_postos_uf ON postos (uf);
CREATE INDEX IF NOT EXISTS idx_interdicoes_cnpj ON interdicoes_anp (posto_cnpj);
CREATE INDEX IF NOT EXISTS idx_pmqc_cnpj ON inspecoes_pmqc (posto_cnpj);
