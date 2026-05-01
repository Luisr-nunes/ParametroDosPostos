-- Seed Postos
INSERT INTO postos (cnpj, razao_social, endereco, municipio, uf, status_autorizacao, localizacao) VALUES
('00.000.000/0001-01', 'Auto Posto Bandeira S/A', 'Av. Paulista, 1000', 'São Paulo', 'SP', 'ATIVO', ST_SetSRID(ST_MakePoint(-46.6560, -23.5617), 4326)),
('11.111.111/0001-11', 'Posto Centro LTDA', 'Rua Augusta, 200', 'São Paulo', 'SP', 'ATIVO', ST_SetSRID(ST_MakePoint(-46.6494, -23.5489), 4326)),
('22.222.222/0001-22', 'Posto Ipiranga Br', 'Av. Brasil, 500', 'Rio de Janeiro', 'RJ', 'INATIVO', ST_SetSRID(ST_MakePoint(-43.2211, -22.8808), 4326)),
('33.333.333/0001-33', 'Comercial de Combustiveis Sul', 'Rua 15 de Novembro, 10', 'Curitiba', 'PR', 'ATIVO', ST_SetSRID(ST_MakePoint(-49.2733, -25.4284), 4326)),
('44.444.444/0001-44', 'Posto Estrela do Norte', 'Av. Norte, 300', 'Recife', 'PE', 'ATIVO', ST_SetSRID(ST_MakePoint(-34.8770, -8.0476), 4326))
ON CONFLICT (cnpj) DO UPDATE SET 
    localizacao = EXCLUDED.localizacao,
    status_autorizacao = EXCLUDED.status_autorizacao;

-- Seed Interdições
INSERT INTO interdicoes_anp (posto_cnpj, motivo, data_interdicao, status) VALUES
('11.111.111/0001-11', 'Bomba fraudada (vazão incorreta)', CURRENT_DATE, 'INTERDITADO'),
('22.222.222/0001-22', 'Qualidade do Etanol Adulterada', CURRENT_DATE, 'DESINTERDITADO');

-- Seed PMQC
INSERT INTO inspecoes_pmqc (posto_cnpj, combustivel, parametro, resultado_obtido, limite_especificacao, conforme, data_coleta) VALUES
('00.000.000/0001-01', 'Gasolina Comum', 'Teor de Etanol', '27%', '27%', true, CURRENT_DATE),
('11.111.111/0001-11', 'Etanol Hidratado', 'Massa Específica', '790', '805', false, CURRENT_DATE);
