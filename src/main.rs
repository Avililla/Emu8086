mod emulator;
pub use emulator::Registers;
fn main() {
    let mut emulator = emulator::Emulator8086::new();
    if let Err(e) = emulator.load_com("hi-world.com") {
        println!("Error al cargar el programa: {:?}", e);
        return;
    }
    emulator.imprimir_estado_memoria(0x0100, 0x0170)
}

//http://atc2.aut.uah.es/~avicente/asignaturas/ects/pdf/ects_t2.pdf
//Manual http://bitsavers.org/components/intel/8086/9800722-03_The_8086_Family_Users_Manual_Oct79.pdf
