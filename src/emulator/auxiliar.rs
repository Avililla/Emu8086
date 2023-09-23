//Flags 
pub const FLAG_CF: u16 = 0b0000_0000_0000_0001;  // Bit 0
pub const FLAG_PF: u16 = 0b0000_0000_0000_0100;  // Bit 2
pub const FLAG_AF: u16 = 0b0000_0000_0001_0000;  // Bit 4
pub const FLAG_ZF: u16 = 0b0000_0000_0100_0000;  // Bit 6
pub const FLAG_SF: u16 = 0b0000_0000_1000_0000;  // Bit 7
pub const FLAG_TF: u16 = 0b0000_0001_0000_0000;  // Bit 8
pub const FLAG_IF: u16 = 0b0000_0010_0000_0000;  // Bit 9
pub const FLAG_DF: u16 = 0b0000_0100_0000_0000;  // Bit 10
pub const FLAG_OF: u16 = 0b0000_1000_0000_0000;  // Bit 11

///ADD///
//Funcion que recibe dos numeros ya sean positivos o negativos y devuelve el resultado de la suma
// si hay overflow y el carry
pub fn add_8bit_complemento_a2(a: u8, b: u8) -> (u8, bool, bool, bool) {
    let (result, overflow) = a.overflowing_add(b);
    let carry = ((a as u16 + b as u16) & 0x0100) != 0;
    let aux = (a & 0xF) + (b & 0xF) > 0xF; // Check for auxiliary flag
    (result, overflow, carry, aux)
}

pub fn add_16bit_complemento_a2(a: u16, b: u16) -> (u16, bool, bool, bool) {
    let (result, overflow) = a.overflowing_add(b);
    let carry = ((a as u32 + b as u32) & 0x0001_0000) != 0;
    let aux = (a & 0xF) + (b & 0xF) > 0xF; // Check for auxiliary flag
    (result, overflow, carry, aux)
}


//Función que actualiza las flags de la operación de suma
pub fn actualizar_flags_add(flags: &mut u16, resultado: u16, overflow: bool, carry: bool, aux: bool) {
    // Sign flag
    *flags &= !(FLAG_CF | FLAG_PF | FLAG_AF | FLAG_ZF | FLAG_SF | FLAG_OF); //Limpiamos las flags
    if (resultado & 0x8000) != 0 {
        *flags |= FLAG_SF;
    } else {
        *flags &= !FLAG_SF;
    }
    
    // Zero flag
    if resultado == 0 {
        *flags |= FLAG_ZF;
    } else {
        *flags &= !FLAG_ZF;
    }

    // Overflow flag
    if overflow {
        *flags |= FLAG_OF;
    } else {
        *flags &= !FLAG_OF;
    }
    
    // Carry flag
    if carry {
        *flags |= FLAG_CF;
    } else {
        *flags &= !FLAG_CF;
    }

    // Parity flag: verificar si el número de bits establecidos en el byte de menor peso es par
    let mut count = 0;
    for i in 0..8 {
        if (resultado & (1 << i)) != 0 {
            count += 1;
        }
    }
    if count % 2 == 0 {
        *flags |= FLAG_PF;
    } else {
        *flags &= !FLAG_PF;
    }
    
    // Auxiliary flag: verifica si hay un carry de los primeros 4 bits
    if aux {
        *flags |= FLAG_AF;
    } else {
        *flags &= !FLAG_AF;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_8bit_complemento_a2() {
        // Test normal sin overflow, carry o aux
        let (result, overflow, carry, aux) = add_8bit_complemento_a2(50, 25);
        assert_eq!(result, 75);
        assert_eq!(overflow, false);
        assert_eq!(carry, false);
        assert_eq!(aux, false);

        // Test con overflow y carry
        let (result, overflow, carry, aux) = add_8bit_complemento_a2(200, 100);
        assert_eq!(result, 44); // 300 % 256
        assert_eq!(overflow, true);
        assert_eq!(carry, true);
        assert_eq!(aux, false);

        // Test con aux
        let (result, overflow, carry, aux) = add_8bit_complemento_a2(0x0F, 0x01);
        assert_eq!(result, 0x10);
        assert_eq!(overflow, false);
        assert_eq!(carry, false);
        assert_eq!(aux, true);
    }

    #[test]
    fn test_add_16bit_complemento_a2() {
        // Test normal sin overflow, carry o aux
        let (result, overflow, carry, aux) = add_16bit_complemento_a2(5000, 2500);
        assert_eq!(result, 7500);
        assert_eq!(overflow, false);
        assert_eq!(carry, false);
        assert_eq!(aux, false);

        // Test con overflow y carry
        let (result, overflow, carry, aux) = add_16bit_complemento_a2(40000, 30000);
        assert_eq!(result, 4464); // 70000 % 65536
        assert_eq!(overflow, true);
        assert_eq!(carry, true);
        assert_eq!(aux, false);

        // Test con aux
        let (result, overflow, carry, aux) = add_16bit_complemento_a2(0x000F, 0x0001);
        assert_eq!(result, 0x0010);
        assert_eq!(overflow, false);
        assert_eq!(carry, false);
        assert_eq!(aux, true);
    }
}
