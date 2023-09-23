use std::fs::File;
use std::io::Read;
use crate::emulator::registers::Registers;

use crate::emulator::auxiliar::*;
const MEM_SIZE: usize = 1 << 20;
const COM_START: usize = 0x7100;
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

    //Decode ModRM
    //addressing_mode,registro destino,origen
    fn decode_modrm(modrm: u8) -> (u8, u8, u8) {
        let addressing_mode = (modrm & 0b11000000) >> 6; 
        let reg = (modrm & 0b00111000) >> 3;
        let rm = modrm & 0b00000111;
        (addressing_mode, reg, rm)
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

    //Reformar el carry y el overflow hay que controlar si son dos numeros negativos
    //ADD Add
    fn add(&mut self,opcode: u8){
        match opcode{
            0x00 =>{

            },
            0x01 =>{

            },
            0x02 =>{
                //Igual que el 3 pero con byte en vez de word
                let mod_rm = self.fetch(); //Leer el byte que nos dice el modo de direccionamiento
                let (mod_field, reg_field, rm_field) = Self::decode_modrm(mod_rm);
                println!("Mod: 0x{:02x}, Reg: 0x{:02x}, R/M: 0x{:02x}", mod_field, reg_field, rm_field);
                match mod_field{
                    0x00=>{
                        let aux = self.fetch();
                    },
                    0x01=>{
                        let aux = self.fetch();
                    },
                    0x02=>{
                        let aux = self.fetch();
                    },
                    0x03 => {
                        //ADD AL,CL registro a registro
                        let origin_register_value = self.registers.get_low_byte(self.registers.get_register_by_index(rm_field));
                        let destination_register_value = self.registers.get_low_byte(self.registers.get_register_by_index(reg_field));
                        let (new_value, overflow) = destination_register_value.overflowing_add(origin_register_value);
                        //Poner el la parte baja de AX el resultado
                        self.registers.ax = (self.registers.ax & 0xFF00) | new_value as u16;
                        self.registers.write_register_by_index(reg_field, (self.registers.get_register_by_index(rm_field)& 0xFF00)| new_value as u16);
                        if overflow {
                            self.registers.flags |= FLAG_OF;
                        }
                    },
                    _ => {}
                }
                
            },
            0x03 =>{
                // 7   6   5   4   3   2   1   0
                // +---+---+---+---+---+---+---+---+
                // | Mod   |   Reg/Opcode  |  R/M   |
                // +---+---+---+---+---+---+---+---+
                let mod_rm = self.fetch(); //Leer el byte que nos dice el modo de direccionamiento
                let (mod_field, reg_field, rm_field) = Self::decode_modrm(mod_rm);
                match mod_field{
                    0x00=>{
                        //Accedemos a memoria usando un registro como indice
                        let origin_register_value = self.registers.get_base_address_from_code(rm_field);
                        let destination_register_value = self.registers.get_register_by_index(reg_field);
                        let memory_value = self.get_w_from_memory(self.registers.ds, origin_register_value);
                        let(new_value,overflow,carry, aux) = add_16bit_complemento_a2(destination_register_value, memory_value);
                        self.registers.write_register_by_index(reg_field, new_value);
                        actualizar_flags_add(&mut self.registers.flags, new_value, overflow, carry, aux);
                        self.pending_cycles += 9;
                    },
                    0x01=>{
                        //Accedemos a memoria usando un registro como indice y un desplazamiento de 8 bits
                        let origin_register_value = self.registers.get_base_address_from_code(rm_field);
                        let displacement = self.fetch();
                        let destination_register_value = self.registers.get_register_by_index(reg_field);
                        let memory_value = self.get_w_from_memory(self.registers.ds, origin_register_value + displacement as u16);
                        let (new_value,overflow,carry,aux) = add_16bit_complemento_a2(destination_register_value, memory_value);
                        self.registers.write_register_by_index(reg_field, new_value);
                        actualizar_flags_add(&mut self.registers.flags, new_value, overflow, carry, aux);
                        self.pending_cycles += 9;
                    },
                    0x02=>{
                        //Accedemos a memoria usando un registro como indice y un desplazamiento de 16 bits
                        let origin_register_value = self.registers.get_base_address_from_code(rm_field);
                        let displacement_l = self.fetch();
                        let displacement_h = self.fetch();
                        let displacement = (displacement_h as u16) << 8 | displacement_l as u16;
                        let destination_register_value = self.registers.get_register_by_index(reg_field);
                        let memory_value = self.get_w_from_memory(self.registers.ds, origin_register_value + displacement);
                        let (new_value,overflow,carry,aux) = add_16bit_complemento_a2(destination_register_value, memory_value);
                        self.registers.write_register_by_index(reg_field, new_value);
                        actualizar_flags_add(&mut self.registers.flags, new_value, overflow, carry, aux);
                        self.pending_cycles += 9;
                    },
                    0x03=>{
                        let origin_register_value = self.registers.get_register_by_index(rm_field);
                        let destination_register_value = self.registers.get_register_by_index(reg_field);
                        let (new_value,overflow,carry,aux) = add_16bit_complemento_a2(destination_register_value, origin_register_value);
                        self.registers.write_register_by_index(reg_field, new_value);
                        actualizar_flags_add(&mut self.registers.flags, new_value, overflow, carry, aux);
                        self.pending_cycles += 3;
                    },
                    _ => {}
                }
            },
            0x04 =>{
                let inmediate_value = self.fetch();
                let al = self.registers.get_low_byte(self.registers.ax);
                let (new_al,overflow,carry, aux) = add_8bit_complemento_a2(al, inmediate_value);
                self.registers.ax = (self.registers.ax & 0xFF00) | new_al as u16;
                actualizar_flags_add(&mut self.registers.flags, new_al as u16, overflow, carry, aux);
                self.pending_cycles += 4;
            },
            0x05 =>{
                let inmediate_value_l = self.fetch();
                let inmediate_value_h = self.fetch();
                let inmediate_value = (inmediate_value_h as u16) << 8 | inmediate_value_l as u16;
                let(new_ax,overflow,carry, aux) = add_16bit_complemento_a2(self.registers.ax, inmediate_value);
                self.registers.ax = new_ax;
                actualizar_flags_add(&mut self.registers.flags, new_ax, overflow, carry, aux);
                self.pending_cycles += 4;
            },
            _ => {}
        }
    }
}