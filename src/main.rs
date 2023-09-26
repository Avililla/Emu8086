mod emulator;
use crate::emulator::emulator::Emulator8086;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path:String;
    if args.len() < 2 {
        println!("Por favor, proporciona la dirección del archivo como argumento.");
        file_path = "noname.com".to_string();
    }else{
        file_path = args[1].to_string();
    }
    println!("Cargando el programa: {}", file_path);
    let mut emulator = Emulator8086::new();
    if let Err(e) = emulator.load_com(&file_path) {
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
// http://www.mathemainzel.info/files/x86asmref.html#xor