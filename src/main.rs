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
