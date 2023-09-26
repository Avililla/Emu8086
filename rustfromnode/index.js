const ffi = require('ffi-napi');
const ref = require('ref-napi');
const Emulator8086Ptr = ref.refType(ref.types.void);

const lib = ffi.Library('../emu8086/target/release/emu8086', {
    'create_emulator': [Emulator8086Ptr, []],
    'destroy_emulator': ['void', [Emulator8086Ptr]],
    'obtener_estado_registros_ffi': ['string', [Emulator8086Ptr]],
    'liberar_cadena': ['void', ['string']]
});

const emulatorInstance = lib.create_emulator();

const estado = lib.obtener_estado_registros_ffi(emulatorInstance);
console.log(estado);

lib.liberar_cadena(estado);
lib.destroy_emulator(emulatorInstance);
