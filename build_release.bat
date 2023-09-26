@echo off

REM Verificar si el directorio 'emulator' existe, si no, crearlo
if not exist 8086-emulator\emulator mkdir 8086-emulator\emulator

cargo build --release --target-dir 8086-emulator\temp
move 8086-emulator\temp\release\* 8086-emulator\emulator\
rmdir /s /q 8086-emulator\temp
pause