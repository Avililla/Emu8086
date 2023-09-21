mod emulator;
fn main() {
    let mut emulator = emulator::Emulator8086::new();
    if let Err(e) = emulator.load_com("noname.com") {
        println!("Error al cargar el programa: {:?}", e);
        return;
    }
    let mut instruction = emulator.fetch(); //Primera instruccion
    while instruction != 0xc3 {
        emulator.decode_and_execute(instruction);
        emulator.imprimir_estado_registros();
        instruction = emulator.fetch();
    }
}

//http://atc2.aut.uah.es/~avicente/asignaturas/ects/pdf/ects_t2.pdf
//Manual http://bitsavers.org/components/intel/8086/9800722-03_The_8086_Family_Users_Manual_Oct79.pdf
// 2-51 Ciclos por instruccion