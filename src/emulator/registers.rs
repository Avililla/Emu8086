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
            ax: 0x0000,
            bx: 0,
            cx: 0x0000,
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
            flags: 0b0000_0010_0000_0000,
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

    pub fn get_base_address_from_code(&self,code: u8)->u16{
        match code{
            0b000 => self.bx+self.si,
            0b001 => self.bx+self.di,
            0b010 => self.bp+self.si,
            0b011 => self.bp+self.di,
            0b100 => self.si,
            0b101 => self.di,
            0b110 => self.bp,
            0b111 => self.bx,
            _ => panic!("Codigo de base no valido: {}", code),
        }
    }
}