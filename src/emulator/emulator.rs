use std::fs::File;
use std::io::Read;
use crate::emulator::registers::Registers;
use crate::emulator::opcodes::*;

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
                }else if OPCODES_AND.contains(&opcode){
                    self.and(opcode)
                }else if OPCODES_MOV.contains(&opcode){
                    self.mov(opcode);
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
    //ADD Add Mirar como funciona ADD mal funcionamiento
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

    fn and(&mut self,opcode: u8){
        match opcode{
            0x20=>{

            },
            0x21=>{

            },
            0x22=>{

            },
            0x23=>{

            },
            0x24=>{
                let inmediate_value = self.fetch();
                let al = self.registers.get_low_byte(self.registers.ax);
                let new_al = al & inmediate_value;
                self.registers.ax = (self.registers.ax & 0xFF00) | new_al as u16;
                actualizar_flags_and(&mut self.registers.flags, new_al as u16);
                self.pending_cycles += 4;
            },
            0x25=>{
                let inmediate_value_l = self.fetch();
                let inmediate_value_h = self.fetch();
                let inmediate_value = (inmediate_value_h as u16) << 8 | inmediate_value_l as u16;
                let new_ax = self.registers.ax & inmediate_value;
                self.registers.ax = new_ax;
                actualizar_flags_and(&mut self.registers.flags, new_ax);
                self.pending_cycles += 4;
            },
            _=>{}
        }
    }

    //TODO: Implementar las instrucciones de mov
    // MOV     rb,rmb      8A mr d0 d1    B   2~4    -------- //implementada
    // MOV     rmb,rb      88 mr d0 d1    B   2~4    -------- //implementada
    // MOV     rmw,rw      89 mr d0 d1    W   2~4    --------
    // MOV     rw,rmw      8B mr d0 d1    W   2~4    --------
    // MOV     rmw,sr      8C mr d0 d1        2~4    --------
    // MOV     sr,rmw      8E mr d0 d1        2~4    --------
    // MOV     AL,rmb      A0 d0 d1      B   3    --------
    // MOV     AX,rmw      A1 d0 d1      W   3    --------
    // MOV     rmb,AL      A2 d0 d1      B   3    --------
    // MOV     rmw,AX      A3 d0 d1      W   3    --------
    // MOV     AL,ib       B0 i0         B   2    --------
    // MOV     CL,ib       B1 i0         B   2    --------
    // MOV     DL,ib       B2 i0         B   2    --------
    // MOV     BL,ib       B3 i0         B   2    --------
    // MOV     AH,ib       B4 i0         B   2    --------
    // MOV     CH,ib       B5 i0         B   2    --------
    // MOV     DH,ib       B6 i0         B   2    --------
    // MOV     BH,ib       B7 i0         B   2    --------
    // MOV     AX,iw       B8 i0 i1      W   3    --------
    // MOV     CX,iw       B9 i0 i1      W   3    --------
    // MOV     DX,iw       BA i0 i1      W   3    --------
    // MOV     BX,iw       BB i0 i1      W   3    --------
    // MOV     SP,iw       BC i0 i1      W   3    --------
    // MOV     BP,iw       BD i0 i1      W   3    --------
    // MOV     SI,iw       BE i0 i1      W   3    --------
    // MOV     DI,iw       BF i0 i1      W   3    --------
    // MOV     rmb,ib      C6 mr d0 d1 i0 B   3~5    --------
    // MOV     rmw,iw      C7 mr d0 d1 i0 i1   W   4~6    --------
    fn mov(&mut self,opcode:u8){
        match opcode{
            0x88=>{
                let mod_rm = self.fetch();
                println!("ModRM: 0x{:02x}", mod_rm);
                // 7   6   5   4   3   2   1   0
                // +---+---+---+---+---+---+---+---+
                // | Mod   |   Reg/Opcode  |  R/M   |
                // +---+---+---+---+---+---+---+---+
                let (mod_field, reg_field, rm_field) = Self::decode_modrm(mod_rm);
                println!("Mod: 0x{:02x}, Reg: 0x{:02x}, R/M: 0x{:02x}", mod_field, reg_field, rm_field);
                let src = self.registers.get_register_by_index_byte(reg_field);
                let dst = self.registers.get_base_address_from_code(rm_field);
                match mod_field{
                    0x00 => {
                        println!("0x00 ejecutando");
                        self.memory[dst as usize] = src as u8;
                        self.pending_cycles += 3;
                    },
                    0x01 => {
                        let aux = self.fetch();
                        println!("Desplazamiento: 0x{:02x}", aux);
                        let dst = dst + aux as u16;
                        println!("Dirección efectiva: 0x{:04x}", dst);
                        self.memory[dst as usize] = src as u8;
                        self.pending_cycles += 3;
                    },
                    0x02 => {
                        let aux_l = self.fetch();
                        let aux_h = self.fetch();
                        let aux = (aux_h as u16) << 8 | aux_l as u16;
                        let dst = dst + aux;
                        self.memory[dst as usize] = src as u8;
                        self.pending_cycles += 3;
                    },
                    0x03 => {
                        println!("Ejecutando 0x03");
                    },
                    _ => {}
                }
            },
            0x8A => {
                let mod_rm = self.fetch();
                let (mod_field, reg_field, rm_field) = Self::decode_modrm(mod_rm);
                match mod_field{
                    0x00 => {
                        println!("0x00 ejecutando");
                        let src = self.registers.get_base_address_from_code(rm_field);
                        self.registers.write_register_by_index_byte(reg_field, self.memory[src as usize]);
                        self.pending_cycles += 3;
                    },
                    0x01 => {
                        println!("0x01 ejecutando");
                        let src = self.registers.get_base_address_from_code(rm_field);
                        let aux = self.fetch();
                        let src = src + aux as u16;
                        self.registers.write_register_by_index_byte(reg_field, self.memory[src as usize]);
                        self.pending_cycles += 3;
                    },
                    0x02 => {
                        println!("0x02 ejecutando");
                        let src = self.registers.get_base_address_from_code(rm_field);
                        let aux_l = self.fetch();
                        let aux_h = self.fetch();
                        let aux = (aux_h as u16) << 8 | aux_l as u16;
                        let src = src + aux;
                        self.registers.write_register_by_index_byte(reg_field, self.memory[src as usize]);
                        self.pending_cycles += 3;
                    },
                    0x03 => {
                        println!("0x03 ejecutando");
                        self.registers.write_register_by_index_byte(reg_field, self.registers.get_register_by_index_byte(rm_field));
                    },
                    _=>{}
                }
            },
            0xB0 => {
                let inmediate_value = self.fetch();
                self.registers.ax = self.registers.write_low_byte(self.registers.ax,inmediate_value);
                self.pending_cycles += 2;
            },
            0xB1 => {
                let inmediate_value = self.fetch();
                self.registers.cx = self.registers.write_low_byte(self.registers.cx,inmediate_value);
                self.pending_cycles += 2;
            },
            0xB2 => {
                let inmediate_value = self.fetch();
                self.registers.dx = self.registers.write_low_byte(self.registers.dx,inmediate_value);
                self.pending_cycles += 2;
            },
            0xB3 => {
                let inmediate_value = self.fetch();
                self.registers.bx = self.registers.write_low_byte(self.registers.bx,inmediate_value);
                self.pending_cycles += 2;
            },
            0xB4 => {
                let inmediate_value = self.fetch();
                self.registers.ax = self.registers.write_high_byte(self.registers.ax,inmediate_value);
                self.pending_cycles += 2;
            },
            0xB5 => {
                let inmediate_value = self.fetch();
                self.registers.cx = self.registers.write_high_byte(self.registers.cx,inmediate_value);
                self.pending_cycles += 2;
            },
            0xB6 => {
                let inmediate_value = self.fetch();
                self.registers.dx = self.registers.write_high_byte(self.registers.dx,inmediate_value);
                self.pending_cycles += 2;
            },
            0xB7 => {
                let inmediate_value = self.fetch();
                self.registers.bx = self.registers.write_high_byte(self.registers.bx,inmediate_value);
                self.pending_cycles += 2;
            },
            0xB8 => {
                let inmediate_value_l = self.fetch();
                let inmediate_value_h = self.fetch();
                let inmediate_value = (inmediate_value_h as u16) << 8 | inmediate_value_l as u16;
                self.registers.ax = inmediate_value;
                self.pending_cycles += 3;
            },
            0xB9 => {
                let inmediate_value_l = self.fetch();
                let inmediate_value_h = self.fetch();
                let inmediate_value = (inmediate_value_h as u16) << 8 | inmediate_value_l as u16;
                self.registers.cx = inmediate_value;
                self.pending_cycles += 3;
            },
            0xBA => {
                let inmediate_value_l = self.fetch();
                let inmediate_value_h = self.fetch();
                let inmediate_value = (inmediate_value_h as u16) << 8 | inmediate_value_l as u16;
                self.registers.dx = inmediate_value;
                self.pending_cycles += 3;
            },
            0xBB => {
                let inmediate_value_l = self.fetch();
                let inmediate_value_h = self.fetch();
                let inmediate_value = (inmediate_value_h as u16) << 8 | inmediate_value_l as u16;
                self.registers.bx = inmediate_value;
                self.pending_cycles += 3;
            },
            0xBC => {
                let inmediate_value_l = self.fetch();
                let inmediate_value_h = self.fetch();
                let inmediate_value = (inmediate_value_h as u16) << 8 | inmediate_value_l as u16;
                self.registers.sp = inmediate_value;
                self.pending_cycles += 3;
            },
            0xBD => {
                let inmediate_value_l = self.fetch();
                let inmediate_value_h = self.fetch();
                let inmediate_value = (inmediate_value_h as u16) << 8 | inmediate_value_l as u16;
                self.registers.bp = inmediate_value;
                self.pending_cycles += 3;
            },
            0xBE => {
                let inmediate_value_l = self.fetch();
                let inmediate_value_h = self.fetch();
                let inmediate_value = (inmediate_value_h as u16) << 8 | inmediate_value_l as u16;
                self.registers.si = inmediate_value;
                self.pending_cycles += 3;
            },
            0xBF => {
                let inmediate_value_l = self.fetch();
                let inmediate_value_h = self.fetch();
                let inmediate_value = (inmediate_value_h as u16) << 8 | inmediate_value_l as u16;
                self.registers.di = inmediate_value;
                self.pending_cycles += 3;
            },
            _=>{}
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn start_emulator(){
        let emulator = Emulator8086::new();
        assert_eq!(emulator.registers.ax, 0);
        assert_eq!(emulator.registers.bx, 0);
        assert!(emulator.memory.iter().all(|&byte| byte == 0));
        assert_eq!(emulator.pending_cycles, 0);
    }

    #[test]
    fn load_com(){
        let mut emulator = Emulator8086::new();
        if let Err(e) = emulator.load_com("./tests/load_com_test.com") {
            panic!("Error al cargar el programa: {:?}", e);
        }
        assert_eq!(emulator.memory[COM_START], 0x05);
        assert_eq!(emulator.memory[COM_START + 1], 0x0F);
        assert_eq!(emulator.memory[COM_START + 2], 0x00);
        assert_eq!(emulator.memory[COM_START + 3], 0xC3);
    }

    #[test]
    fn test_mov_inm_low(){
        let mut emulator = Emulator8086::new();
        if let Err(e) = emulator.load_com("./tests/mov/MOV_LOW_REG.com") {
            panic!("Error al cargar el programa: {:?}", e);
        }
        let mut instruction = emulator.fetch(); //Primera instruccion
        while instruction != 0xc3 {
            emulator.decode_and_execute(instruction);
            emulator.imprimir_estado_registros();
            instruction = emulator.fetch();
        }
        //Test mov de inmediatos a registros bajos
        assert_eq!(emulator.registers.ax, 0x0011);
        assert_eq!(emulator.registers.bx, 0x0011);
        assert_eq!(emulator.registers.cx, 0x0011);
        assert_eq!(emulator.registers.dx, 0x0011);
    }

    #[test]
    fn test_mov_inm_high(){
        let mut emulator = Emulator8086::new();
        if let Err(e) = emulator.load_com("./tests/mov/MOV_HIGH_REG.com") {
            panic!("Error al cargar el programa: {:?}", e);
        }
        let mut instruction = emulator.fetch(); //Primera instruccion
        while instruction != 0xc3 {
            emulator.decode_and_execute(instruction);
            emulator.imprimir_estado_registros();
            instruction = emulator.fetch();
        }
        //Test mov de inmediatos a registros altos
        assert_eq!(emulator.registers.ax, 0x1100);
        assert_eq!(emulator.registers.bx, 0x1100);
        assert_eq!(emulator.registers.cx, 0x1100);
        assert_eq!(emulator.registers.dx, 0x1100);
    }
}