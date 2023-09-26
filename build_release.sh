#!/bin/bash

# Verificar si el directorio 'emulator' existe, si no, crearlo
if [ ! -d "8086-emulator/emulator" ]; then
    mkdir 8086-emulator/emulator
fi

cargo build --release --target-dir 8086-emulator/temp
mv 8086-emulator/temp/release/* 8086-emulator/emulator/
rm -r 8086-emulator/temp