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
            di: 0x0000,
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

    //Solo se usan para cuando son la base de la instrucción

    //Según el indice en el modo de redireccionamiento devuelve el valor //Correcto
    pub fn get_register_by_index(&self,index:u8)->u16{
        match index{
            0b000 => self.ax,
            0b001 => self.cx,
            0b010 => self.dx,
            0b011 => self.bx,
            0b100 => self.sp,
            0b101 => self.bp,
            0b110 => self.si,
            0b111 => self.di,
            _ => panic!("Indice de registro no valido: {}", index),
        }
    }

    //Según el indice en el modo de redireccionamiento escribe el valor //Correcto
    pub fn write_register_by_index(&mut self, index: u8, value: u16){
        match index{
            0b000 => self.ax = value,
            0b001 => self.cx = value,
            0b010 => self.dx = value,
            0b011 => self.bx = value,
            0b100 => self.sp = value,
            0b101 => self.bp = value,
            0b110 => self.si = value,
            0b111 => self.di = value,
            _ => panic!("Indice de registro no valido: {}", index),
        }
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

    pub fn get_base_address_from_code(&self,code: u8)->u16{
        match code{
            0b000 => self.registers.bx+self.registers.si,
            0b001 => self.registers.bx+self.registers.di,
            0b010 => self.registers.bp+self.registers.si,
            0b011 => self.registers.bp+self.registers.di,
            0b100 => self.registers.si,
            0b101 => self.registers.di,
            0b110 => self.registers.bp,
            0b111 => self.registers.bx,
            _ => panic!("Codigo de base no valido: {}", code),
        }
    }

    pub fn fetch(&mut self)->u8{
        let base_address = (self.registers.cs as usize) << 4;
        let offset = self.registers.ip as usize;
        let effective_address = base_address + offset;
        self.registers.ip += 1;
        self.memory[effective_address] //Leer 1 byte de memoria del 8086 tarda 4 ciclos de reloj
    }

    //Coger un byte de memoria
    pub fn get_b_from_memory(&self, base:u16, offset:u16)->u8{
        let base_address = (base as usize) << 4;
        let effective_address = base_address + (offset as usize);
        self.memory[effective_address]
    }

    //Coger un word de memoria
    pub fn get_w_from_memory(&self, base:u16, offset:u16)->u16{
        let base_address = (base as usize) << 4;
        let effective_address = base_address + (offset as usize);
        println!("Direccion efectiva: 0x{:04x}", effective_address);
        let low_byte = self.memory[effective_address];
        let high_byte = self.memory[effective_address + 1];
        (high_byte as u16) << 8 | low_byte as u16
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
        println!("+-----+-----+-----+-----+-----+-----+-----+-----+");
        println!(
            "| OF:{} | DF:{} | IF:{} | TF:{} | SF:{} | ZF:{} | AF:{} | PF:{} |",
            if (self.registers.flags & FLAG_OF) != 0 { "1" } else { "0" },
            if (self.registers.flags & FLAG_DF) != 0 { "1" } else { "0" },
            if (self.registers.flags & FLAG_IF) != 0 { "1" } else { "0" },
            if (self.registers.flags & FLAG_TF) != 0 { "1" } else { "0" },
            if (self.registers.flags & FLAG_SF) != 0 { "1" } else { "0" },
            if (self.registers.flags & FLAG_ZF) != 0 { "1" } else { "0" },
            if (self.registers.flags & FLAG_AF) != 0 { "1" } else { "0" },
            if (self.registers.flags & FLAG_PF) != 0 { "1" } else { "0" },
        );
        println!("+-----+-----+-----+-----+-----+-----+-----+-----+");
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
                // 7   6   5   4   3   2   1   0
                // +---+---+---+---+---+---+---+---+
                // | Mod   |   Reg/Opcode  |  R/M   |
                // +---+---+---+---+---+---+---+---+
                let mod_rm = self.fetch(); //Leer el byte que nos dice el modo de direccionamiento
                let mod_field = (mod_rm & 0xC0) >> 6;//Determinan si el operando es un registro o un valor en memoria
                let reg_field = (mod_rm & 0x38) >> 3;//Determinan el registro de destino
                let rm_field = mod_rm & 0x07;//Determinan el registro de origen
                match mod_field{
                    0x00=>{
                        //Accedemos a memoria usando un registro como indice
                        let origin_register_value = self.get_base_address_from_code(rm_field);
                        let destination_register_value = self.registers.get_register_by_index(reg_field);
                        let memory_value = self.get_w_from_memory(self.registers.ds, origin_register_value);
                        let (new_value, carry) = destination_register_value.overflowing_add(memory_value);
                        let overflow = ((destination_register_value as i16).overflowing_add(memory_value as i16)).1;
                        self.registers.write_register_by_index(reg_field, new_value);
                        self.registers.flags &= !(FLAG_CF | FLAG_PF | FLAG_AF | FLAG_ZF | FLAG_SF | FLAG_OF);  // Limpiar flags relevantes
                        if carry {
                            self.registers.flags |= FLAG_CF;
                        }
                        if new_value % 2 == 0 {
                            self.registers.flags |= FLAG_PF;
                        }
                        if (new_value & 0x0F) + (memory_value & 0x0F) > 0x0F {
                            self.registers.flags |= FLAG_AF;
                        }
                        if new_value == 0 {
                            self.registers.flags |= FLAG_ZF;
                        }
                        if new_value & 0x8000 != 0 {
                            self.registers.flags |= FLAG_SF;
                        }
                        if overflow {
                            self.registers.flags |= FLAG_OF;
                        }
                        self.pending_cycles += 9;
                    },
                    0x01=>{
                        //Accedemos a memoria usando un registro como indice y un desplazamiento de 8 bits
                        let origin_register_value = self.get_base_address_from_code(rm_field);
                        let displacement = self.fetch();
                        let destination_register_value = self.registers.get_register_by_index(reg_field);
                        let memory_value = self.get_w_from_memory(self.registers.ds, origin_register_value + displacement as u16);
                        let (new_value, carry) = destination_register_value.overflowing_add(memory_value);
                        let overflow = ((destination_register_value as i16).overflowing_add(memory_value as i16)).1;
                        self.registers.write_register_by_index(reg_field, new_value);
                        self.registers.flags &= !(FLAG_CF | FLAG_PF | FLAG_AF | FLAG_ZF | FLAG_SF | FLAG_OF);  // Limpiar flags relevantes
                        if carry {
                            self.registers.flags |= FLAG_CF;
                        }
                        if new_value % 2 == 0 {
                            self.registers.flags |= FLAG_PF;
                        }
                        if (new_value & 0x0F) + (memory_value & 0x0F) > 0x0F {
                            self.registers.flags |= FLAG_AF;
                        }
                        if new_value == 0 {
                            self.registers.flags |= FLAG_ZF;
                        }
                        if new_value & 0x8000 != 0 {
                            self.registers.flags |= FLAG_SF;
                        }
                        if overflow {
                            self.registers.flags |= FLAG_OF;
                        }
                        self.pending_cycles += 9;
                    },
                    0x02=>{
                        //Accedemos a memoria usando un registro como indice y un desplazamiento de 16 bits
                        let origin_register_value = self.get_base_address_from_code(rm_field);
                        let displacement_l = self.fetch();
                        let displacement_h = self.fetch();
                        let displacement = (displacement_h as u16) << 8 | displacement_l as u16;
                        let destination_register_value = self.registers.get_register_by_index(reg_field);
                        let memory_value = self.get_w_from_memory(self.registers.ds, origin_register_value + displacement);
                        let (new_value, carry) = destination_register_value.overflowing_add(memory_value);
                        let overflow = ((destination_register_value as i16).overflowing_add(memory_value as i16)).1;
                        self.registers.write_register_by_index(reg_field, new_value);
                        self.registers.flags &= !(FLAG_CF | FLAG_PF | FLAG_AF | FLAG_ZF | FLAG_SF | FLAG_OF);  // Limpiar flags relevantes
                        if carry {
                            self.registers.flags |= FLAG_CF;
                        }
                        if new_value % 2 == 0 {
                            self.registers.flags |= FLAG_PF;
                        }
                        if (new_value & 0x0F) + (memory_value & 0x0F) > 0x0F {
                            self.registers.flags |= FLAG_AF;
                        }
                        if new_value == 0 {
                            self.registers.flags |= FLAG_ZF;
                        }
                        if new_value & 0x8000 != 0 {
                            self.registers.flags |= FLAG_SF;
                        }
                        if overflow {
                            self.registers.flags |= FLAG_OF;
                        }
                        self.pending_cycles += 9;
                    },
                    0x03=>{
                        let origin_register_value = self.registers.get_register_by_index(rm_field);
                        let destination_register_value = self.registers.get_register_by_index(reg_field);
                        let (new_value, carry) = destination_register_value.overflowing_add(origin_register_value);
                        let overflow = ((destination_register_value as i16).overflowing_add(origin_register_value as i16)).1;
                        self.registers.write_register_by_index(reg_field, new_value);
                        self.registers.flags &= !(FLAG_CF | FLAG_PF | FLAG_AF | FLAG_ZF | FLAG_SF | FLAG_OF);  // Limpiar flags relevantes
                        if carry {
                            self.registers.flags |= FLAG_CF;
                        }
                        if new_value % 2 == 0 {
                            self.registers.flags |= FLAG_PF;
                        }
                        if (new_value & 0x0F) + (origin_register_value & 0x0F) > 0x0F {
                            self.registers.flags |= FLAG_AF;
                        }
                        if new_value == 0 {
                            self.registers.flags |= FLAG_ZF;
                        }
                        if new_value & 0x8000 != 0 {
                            self.registers.flags |= FLAG_SF;
                        }
                        if overflow {
                            self.registers.flags |= FLAG_OF;
                        }
                        self.pending_cycles += 3;
                    },
                    _ => {}
                }
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
                let inmediate_value_l = self.fetch();
                let inmediate_value_h = self.fetch();
                let inmediate_value = (inmediate_value_h as u16) << 8 | inmediate_value_l as u16;
                let (new_ax, carry) = self.registers.ax.overflowing_add(inmediate_value);
                let overflow = ((self.registers.ax as i16).overflowing_add(inmediate_value as i16)).1;
                self.registers.ax = new_ax;
                self.registers.flags &= !(FLAG_CF | FLAG_PF | FLAG_AF | FLAG_ZF | FLAG_SF | FLAG_OF);  // Limpiar flags relevantes
                if carry {
                    self.registers.flags |= FLAG_CF;
                }
                if (new_ax & 0xFF).count_ones() % 2 == 0 {
                    self.registers.flags |= FLAG_PF;
                }
                
                if (new_ax & 0x0F) + (inmediate_value & 0x0F) > 0x0F {
                    self.registers.flags |= FLAG_AF;
                }
        
                if new_ax == 0 {
                    self.registers.flags |= FLAG_ZF;
                }
        
                if new_ax & 0x8000 != 0 {
                    self.registers.flags |= FLAG_SF;
                }
        
                if overflow {
                    self.registers.flags |= FLAG_OF;
                }
                self.pending_cycles += 4;
            },
            _ => {}
        }
    }
    
}