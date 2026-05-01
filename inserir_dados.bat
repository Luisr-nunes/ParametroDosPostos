@echo off
echo Iniciando Ingestor de Dados (modo isolado contra erros 32)...
cd scraper_anp
set CARGO_TARGET_DIR=%TEMP%\parametrodospostos_target_ingest
cargo run --bin ingest_postos
pause
