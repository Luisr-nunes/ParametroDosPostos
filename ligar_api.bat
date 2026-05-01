@echo off
echo Iniciando API (modo isolado contra erros 32)...
set CARGO_TARGET_DIR=%TEMP%\parametrodospostos_target_api
cargo run --bin api
pause
