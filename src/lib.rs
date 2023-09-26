mod emulator;
use crate::emulator::emulator::Emulator8086;
use std::ffi::CString;
use std::os::raw::c_char;
use crate::emulator::auxiliar::*;

#[no_mangle]
pub extern "C" fn create_emulator() -> *mut Emulator8086{
    let emulator = Emulator8086::new();
    Box::into_raw(Box::new(emulator))
}

#[no_mangle]
pub extern "C" fn destroy_emulator(ptr: *mut Emulator8086) {
    if !ptr.is_null() {
        unsafe {
            Box::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn obtener_estado_registros_ffi(ptr: *mut Emulator8086) -> *mut c_char {
    let emulator = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let estado = obtener_estado_registros(emulator);
    let c_str = CString::new(estado).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn liberar_cadena(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            CString::from_raw(ptr);
        }
    }
}

fn obtener_estado_registros(emulator: &Emulator8086) -> String {
    let mut estado = String::new();
    estado.push_str("Estado de los registros\n");
    estado.push_str(&format!("AX: 0x{:04x}\n", emulator.registers.ax));
    estado.push_str(&format!("BX: 0x{:04x}\n", emulator.registers.bx));
    estado.push_str(&format!("CX: 0x{:04x}\n", emulator.registers.cx));
    estado.push_str(&format!("DX: 0x{:04x}\n", emulator.registers.dx));
    estado.push_str(&format!("SI: 0x{:04x}\n", emulator.registers.si));
    estado.push_str(&format!("DI: 0x{:04x}\n", emulator.registers.di));
    estado.push_str(&format!("BP: 0x{:04x}\n", emulator.registers.bp));
    estado.push_str(&format!("SP: 0x{:04x}\n", emulator.registers.sp));
    estado.push_str(&format!("CS: 0x{:04x}\n", emulator.registers.cs));
    estado.push_str(&format!("DS: 0x{:04x}\n", emulator.registers.ds));
    estado.push_str(&format!("ES: 0x{:04x}\n", emulator.registers.es));
    estado.push_str(&format!("SS: 0x{:04x}\n", emulator.registers.ss));
    estado.push_str(&format!("IP: 0x{:04x}\n", emulator.registers.ip));
    // ... (haz lo mismo para los otros registros)
    estado.push_str("+-----+-----+-----+-----+-----+-----+-----+-----+\n");
    estado.push_str("|  OF |  DF |  IF |  TF |  SF |  ZF |  AF |  PF |\n");
    estado.push_str(&format!("|  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |\n",
        if (emulator.registers.flags & FLAG_OF) != 0 { "1" } else { "0" },
        if (emulator.registers.flags & FLAG_DF) != 0 { "1" } else { "0" },
        if (emulator.registers.flags & FLAG_IF) != 0 { "1" } else { "0" },
        if (emulator.registers.flags & FLAG_TF) != 0 { "1" } else { "0" },
        if (emulator.registers.flags & FLAG_SF) != 0 { "1" } else { "0" },
        if (emulator.registers.flags & FLAG_ZF) != 0 { "1" } else { "0" },
        if (emulator.registers.flags & FLAG_AF) != 0 { "1" } else { "0" },
        if (emulator.registers.flags & FLAG_PF) != 0 { "1" } else { "0" },
    ));
    estado.push_str("+-----+-----+-----+-----+-----+-----+-----+-----+\n");
    estado
}
