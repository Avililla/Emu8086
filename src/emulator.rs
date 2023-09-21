use std::fs::File;
use std::io::Read;
const MEM_SIZE: usize = 1 << 20;
const COM_START: usize = 0x7100;

//Banderas del registro de estado para poderlas usar de manera mas simple
const FLAG_CF: u16 = 0b0000_0000_0000_0001;  // Bit 0
const FLAG_PF: u16 = 0b0000_0000_0000_0100;  // Bit 2
const FLAG_AF: u16 = 0b0000_0000_0001_0000;  // Bit 4
const FLAG_ZF: u16 = 0b0000_0000_0100_0000;  // Bit 6
const FLAG_SF: u16 = 0b0000_0000_1000_0000;  // Bit 7
const FLAG_TF: u16 = 0b0000_0001_0000_0000;  // Bit 8
const FLAG_IF: u16 = 0b0000_0010_0000_0000;  // Bit 9
const FLAG_DF: u16 = 0b0000_0100_0000_0000;  // Bit 10
const FLAG_OF: u16 = 0b0000_1000_0000_0000;  // Bit 11

pub struct Registers {
    // Registros generales de 16 bits
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,
    // Registros de índice
    pub si: u16,
    pub di: u16,
    // Registros de segmento
    pub cs: u16,
    pub ds: u16,
    pub ss: u16,
    pub es: u16,
    // Registros de puntero y otros
    pub sp: u16,
    pub bp: u16,
    pub ip: u16,
    // Registro de estado (Flags)
    pub flags: u16,
}

impl Registers{
    pub fn initialize()->Self{
        Self{
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            si: 0,
            di: 0,
            sp: 0xFFFE, // El puntero de pila por lo general empieza en el tope
            bp: 0,
            cs: 0x0700,
            ds: 0x0700,
            ss: 0x0000,
            es: 0x0700,
            ip: 0x0100, // El punto de entrada por defecto para programas
            flags: 0,
        }
    }

    pub fn get_high_byte<T: Into<u16>>(&self, register: T)->u8{
        let value = register.into();
        ((value & 0xFF00) >> 8) as u8 // Hay que desplazar de la parte alta a la baja para que cuando se convierta a 8 bits se quede la parte alta
    }

    pub fn get_low_byte<T: Into<u16>>(&self, register: T)->u8{
        let value = register.into();
        (value & 0x00FF) as u8
    }
}


pub struct Emulator8086 {
    // Registros
    pub registers: Registers,
    // Memoria
    pub memory: Vec<u8>,
    //Ciclos de espera pendientes de emular
    pub pending_cycles: u64,
}

impl Emulator8086{
    pub fn new()->Self{
        Self{
            registers: Registers::initialize(),
            memory: vec![0; MEM_SIZE],
            pending_cycles: 0,
        }
    }

    pub fn load_com(&mut self, path: & str)-> std::io::Result<()> {
        let mut archivo = File::open(path)?;
        let mut buffer = Vec::new();
        archivo.read_to_end(&mut buffer)?;
        for (i, &byte) in buffer.iter().enumerate() {
            self.memory[COM_START + i] = byte;
        }
        Ok(())
    }

    //Función que se ejecuta después de cada instrucción para la emulación del retardo
    pub fn run_pending_cycles(&self){
        
    }

    pub fn fetch(&mut self)->u8{
        let base_address = (self.registers.cs as usize) << 4;
        let offset = self.registers.ip as usize;
        let effective_address = base_address + offset;
        self.registers.ip += 1;
        self.memory[effective_address] //Leer 1 byte de memoria del 8086 tarda 4 ciclos de reloj
    }

    pub fn decode_and_execute(&mut self, opcode: u8){
        match opcode {
            0x37 => self.aaa(),
            0xD5 => self.aad(),
            0xD4 => self.aam(),
            0x3F => self.aas(),
            _ => {
                if (0x00..=0x05).contains(&opcode){
                    self.add(opcode);
                }else if (0x10..=0x15).contains(&opcode){
                    self.adc(opcode)
                } 
                else {
                    panic!("Opcode no implementado: 0x{:02x}", opcode);
                }
            }
        }
    }

    pub fn imprimir_estado_memoria(&self, inicio: usize, fin: usize) {
        println!("Estado de la memoria desde 0x{:04x} hasta 0x{:04x}:", inicio, fin);

        for i in inicio..=fin {
            print!("0x{:02x} ", self.memory[i]);
            if (i - inicio + 1) % 16 == 0 {
                println!(); // Imprime una nueva línea cada 16 bytes para que sea más legible
            }
        }

        println!();
    }

    pub fn imprimir_estado_registros(&self){
        println!("Estado de los registros");
        println!("AX: 0x{:04x}", self.registers.ax);
        println!("BX: 0x{:04x}", self.registers.bx);
        println!("CX: 0x{:04x}", self.registers.cx);
        println!("DX: 0x{:04x}", self.registers.dx);
        println!("SI: 0x{:04x}", self.registers.si);
        println!("DI: 0x{:04x}", self.registers.di);
        println!("SP: 0x{:04x}", self.registers.sp);
        println!("BP: 0x{:04x}", self.registers.bp);
        println!("CS: 0x{:04x}", self.registers.cs);
        println!("DS: 0x{:04x}", self.registers.ds);
        println!("SS: 0x{:04x}", self.registers.ss);
        println!("ES: 0x{:04x}", self.registers.es);
        println!("IP: 0x{:04x}", self.registers.ip);
        println!("Flags: 0x{:04x}", self.registers.flags);
    }

    //Implementación de las microinstrucciones
    //AAA ASCII adjust for addition

    fn aaa(&mut self){
        let mut al = self.registers.get_low_byte(self.registers.ax);
        let mut ah = self.registers.get_high_byte(self.registers.ax);
        if(al & 0x0F > 9) || ((self.registers.flags & FLAG_AF) != 0){
            ah = ah.wrapping_add(1);
            al = al.wrapping_add(6);
            self.registers.flags |= FLAG_AF;
            self.registers.flags |= FLAG_CF;
        }else{
            self.registers.flags &= !FLAG_AF;
            self.registers.flags &= !FLAG_CF;
        }
        al &= 0x0F;
        self.registers.ax = (ah as u16) << 8 | al as u16;
        self.pending_cycles += 4;
    }


    //AAD ASCII adjust for division
    fn aad(&mut self){
        let ah = (self.registers.ax & 0xFF00) >> 8;
        let al = self.registers.ax & 0x00FF;
        let temp = al.wrapping_add(ah*0xA)&0xFF00;
        self.registers.ax = (self.registers.ax & 0xFF00) | temp;
        self.registers.ax &= 0x00FF;
        self.registers.flags &= !(FLAG_PF | FLAG_SF | FLAG_ZF);
        if temp == 0{
            self.registers.flags |= FLAG_ZF;
        }
        self.registers.ax = (ah as u16) << 8 | al as u16;
        self.pending_cycles += 60;
    }

    //AAM ASCII adjust for multiplication
    fn aam(&mut self){
        let mut al = self.registers.get_low_byte(self.registers.ax);
        let mut ah = self.registers.get_high_byte(self.registers.ax);
        ah = al / 0xA;
        al = al % 0xA;
        self.registers.flags &= !(FLAG_PF | FLAG_SF | FLAG_ZF);
        if al == 0 {self.registers.flags |= FLAG_ZF;}
        if al & 0x80 != 0 {
            self.registers.flags |= FLAG_SF;  // Establecer SF si el bit más significativo de AL es 1
        }
        let parity = al.count_ones();
        if parity % 2 == 0 {
            self.registers.flags |= FLAG_PF;
        }
        self.registers.ax = (ah as u16) << 8 | al as u16;
        self.pending_cycles += 83;
    }

    //AAS ASCII adjust for subtraction
    fn aas(&mut self){
        let mut al = self.registers.get_low_byte(self.registers.ax);
        let mut ah = self.registers.get_high_byte(self.registers.ax);
        if (al & 0x0F > 9) || (self.registers.flags & FLAG_AF != 0) {
            al = al.wrapping_sub(6);
            ah = ah.wrapping_sub(1);
            self.registers.flags |= FLAG_AF | FLAG_CF;
        }else{
            self.registers.flags &= !(FLAG_AF | FLAG_CF);
        }
        al &= 0x0F;
        self.registers.ax = (ah as u16) << 8 | al as u16;
        self.pending_cycles += 4;
    }

    //ADC Add with carry
    fn adc(&mut self, opcode: u8){

    }

    //ADD Add
    fn add(&mut self,opcode: u8){
        match opcode{
            0x00 =>{

            },
            0x01 =>{

            },
            0x02 =>{

            },
            0x03 =>{

            },
            0x04 =>{
                let inmediate_value = self.fetch();
                let (new_al, carry) = self.registers.get_low_byte(self.registers.ax).overflowing_add(inmediate_value);
                let overflow = ((self.registers.get_low_byte(self.registers.ax) as i8).overflowing_add(inmediate_value as i8)).1;
                self.registers.ax = (self.registers.ax & 0xFF00) | new_al as u16;
                self.registers.flags &= !(FLAG_CF | FLAG_PF | FLAG_AF | FLAG_ZF | FLAG_SF | FLAG_OF);  // Limpiar flags relevantes
                if carry {
                    self.registers.flags |= FLAG_CF;
                }
                if new_al % 2 == 0 {
                    self.registers.flags |= FLAG_PF;
                }
                if (new_al & 0x0F) + (inmediate_value & 0x0F) > 0x0F {
                    self.registers.flags |= FLAG_AF;
                }
                if new_al == 0 {
                    self.registers.flags |= FLAG_ZF;
                } 
                
                if new_al & 0x80 != 0 {
                    self.registers.flags |= FLAG_SF;
                }
        
                if overflow {
                    self.registers.flags |= FLAG_OF;
                }
                self.pending_cycles += 4;
            },
            0x05 =>{

            },
            _ => {}
        }
    }
    
}