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

    pub fn write_high_byte<T>(&self,register: T, value: u8) -> u16
    where
        T: Into<u16> {
        let current_value: u16 = register.into(); // Convirtiendo a u16
        let cleared_high_byte = current_value & 0x00FF; // Borrar el byte alto
        cleared_high_byte | ((value as u16) << 8) // Devuelve el valor con el nuevo byte alto
    }

    pub fn write_low_byte<T>(&self,register: T, value: u8) -> u16
    where
        T: Into<u16> {
        let current_value: u16 = register.into(); // Convirtiendo a u16
        let cleared_low_byte = current_value & 0xFF00; // Borrar el byte bajo
        cleared_low_byte | (value as u16) // Devuelve el valor con el nuevo byte bajo
    }
    

    //Solo se usan para cuando son la base de la instrucción

    //Según el indice en el modo de redireccionamiento devuelve el valor //Correcto
    // pub fn get_register_by_index(&self,index:u8)->u16{
    //     //Print del registro que se va a leer
    //     match index{
    //         0b000 => self.ax,
    //         0b001 => self.cx,
    //         0b010 => self.dx,
    //         0b011 => self.bx,
    //         0b100 => self.sp,
    //         0b101 => self.bp,
    //         0b110 => self.si,
    //         0b111 => self.di,
    //         _ => panic!("Indice de registro no valido: {}", index),
    //     }
    // }

    //Se usa para saber el registro fuente desde el cual cogemos datos por ejemplo MOV AX, BX esta funcion nos sirve para saber BX
    pub fn get_register_by_index(&self, index: u8) -> u16 {
        match index {
            0b000 => {
                println!("Accediendo al registro AX");
                self.ax
            },
            0b001 => {
                println!("Accediendo al registro CX");
                self.cx
            },
            0b010 => {
                println!("Accediendo al registro DX");
                self.dx
            },
            0b011 => {
                println!("Accediendo al registro BX");
                self.bx
            },
            0b100 => {
                println!("Accediendo al registro SP");
                self.sp
            },
            0b101 => {
                println!("Accediendo al registro BP");
                self.bp
            },
            0b110 => {
                println!("Accediendo al registro SI");
                self.si
            },
            0b111 => {
                println!("Accediendo al registro DI");
                self.di
            },
            _ => panic!("Índice de registro no válido: {}", index),
        }
    }

    //Devuelve la parte baja o alta del registro según el indice solo AX,BX,CX,DX
    pub fn get_register_by_index_byte(&self, index: u8)->u8{
        match index{
            0b000 => {
                println!("Accediendo a la parte baja del registro AX");
                self.get_low_byte(self.ax)
            },
            0b001 => {
                println!("Accediendo a la parte baja del registro CX");
                self.get_low_byte(self.cx)
            },
            0b010 => {
                println!("Accediendo a la parte baja del registro DX");
                self.get_low_byte(self.dx)
            },
            0b011 => {
                println!("Accediendo a la parte baja del registro BX");
                self.get_low_byte(self.bx)
            },
            0b100 => {
                println!("Accediendo a la parte alta del registro AX");
                self.get_high_byte(self.ax)
            },
            0b101 => {
                println!("Accediendo a la parte alta del registro CX");
                self.get_high_byte(self.cx)
            },
            0b110 => {
                println!("Accediendo a la parte alta del registro DX");
                self.get_high_byte(self.dx)
            },
            0b111 => {
                println!("Accediendo a la parte alta del registro BX");
                self.get_high_byte(self.bx)
            },
            _ => panic!("Indice de registro no valido: {}", index),
        }
    }
    
    //Se usa para saber el registro destino al cual vamos a escribir por ejemplo MOV AX, BX esta funcion nos sirve para saber AX
    pub fn write_register_by_index(&mut self, index: u8, value: u16){
        match index {
            0b000 => {
                println!("Escribiendo en el registro AX el valor: {}", value);
                self.ax = value
            },
            0b001 => {
                println!("Escribiendo en el registro CX el valor: {}", value);
                self.cx = value
            },
            0b010 => {
                println!("Escribiendo en el registro DX el valor: {}", value);
                self.dx = value
            },
            0b011 => {
                println!("Escribiendo en el registro BX el valor: {}", value);
                self.bx = value
            },
            0b100 => {
                println!("Escribiendo en el registro SP el valor: {}", value);
                self.sp = value
            },
            0b101 => {
                println!("Escribiendo en el registro BP el valor: {}", value);
                self.bp = value
            },
            0b110 => {
                println!("Escribiendo en el registro SI el valor: {}", value);
                self.si = value
            },
            0b111 => {
                println!("Escribiendo en el registro DI el valor: {}", value);
                self.di = value
            },
            _ => panic!("Índice de registro no válido: {}", index),
        }
    }

    pub fn write_register_by_index_byte(&mut self, index: u8, value: u8){
        match index{
            0b000 => {
                println!("Escribiendo en la parte baja del registro AX el valor: {}", value);
                self.ax = (self.ax & 0xFF00) | value as u16
            },
            0b001 => {
                println!("Escribiendo en la parte baja del registro CX el valor: {}", value);
                self.cx = (self.cx & 0xFF00) | value as u16
            },
            0b010 => {
                println!("Escribiendo en la parte baja del registro DX el valor: {}", value);
                self.dx = (self.dx & 0xFF00) | value as u16
            },
            0b011 => {
                println!("Escribiendo en la parte baja del registro BX el valor: {}", value);
                self.bx = (self.bx & 0xFF00) | value as u16
            },
            0b100 => {
                println!("Escribiendo en la parte alta del registro AX el valor: {}", value);
                self.ax = (self.ax & 0x00FF) | (value as u16) << 8
            },
            0b101 => {
                println!("Escribiendo en la parte alta del registro CX el valor: {}", value);
                self.cx = (self.cx & 0x00FF) | (value as u16) << 8
            },
            0b110 => {
                println!("Escribiendo en la parte alta del registro DX el valor: {}", value);
                self.dx = (self.dx & 0x00FF) | (value as u16) << 8
            },
            0b111 => {
                println!("Escribiendo en la parte alta del registro BX el valor: {}", value);
                self.bx = (self.bx & 0x00FF) | (value as u16) << 8
            },
            _ => panic!("Indice de registro no valido: {}", index),
        }
    }

    //Se usa para saber el registro destino al cual vamos a escribir por ejemplo MOV [BX+0x10h], esta función nos sirve para saber BX
    pub fn get_base_address_from_code(&self,code: u8)->u16{
        match code {
            0b000 => {
                println!("Obteniendo dirección base usando BX+SI: {}", self.bx + self.si);
                self.bx + self.si
            },
            0b001 => {
                println!("Obteniendo dirección base usando BX+DI: {}", self.bx + self.di);
                self.bx + self.di
            },
            0b010 => {
                println!("Obteniendo dirección base usando BP+SI: {}", self.bp + self.si);
                self.bp + self.si
            },
            0b011 => {
                println!("Obteniendo dirección base usando BP+DI: {}", self.bp + self.di);
                self.bp + self.di
            },
            0b100 => {
                println!("Obteniendo dirección base usando SI: {}", self.si);
                self.si
            },
            0b101 => {
                println!("Obteniendo dirección base usando DI: {}", self.di);
                self.di
            },
            0b110 => {
                println!("Obteniendo dirección base usando BP: {}", self.bp);
                self.bp
            },
            0b111 => {
                println!("Obteniendo dirección base usando BX: {}", self.bx);
                self.bx
            },
            _ => panic!("Código de base no válido: {}", code),
        }
    }
}