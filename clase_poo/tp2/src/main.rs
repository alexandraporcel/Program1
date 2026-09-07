struct Numero {
    valor: u64,
}

impl Numero {
    // 1. ELIMINAR DÍGITO
    
    fn eliminar_digito(&mut self, digito_a_eliminar: u64) {
        let mut temporal = self.valor;
        let mut nuevo_numero = 0;
        let mut multiplicador = 1;

        while temporal > 0 {
            let digito = temporal % 10;
            if digito != digito_a_eliminar {
                nuevo_numero = nuevo_numero + (digito * multiplicador);
                multiplicador = multiplicador * 10;
            }
            temporal = temporal / 10;
        }
        self.valor = nuevo_numero;
    }

    // 2. CONVERTIR A BINARIO
    fn a_binario(&self) -> u64 {
        let mut temporal = self.valor;
        let mut resultado_binario = 0;
        let mut multiplicador = 1;

        
        while temporal > 0 {
            let residuo = temporal % 2; 
            
            resultado_binario = resultado_binario + (residuo * multiplicador);
            multiplicador = multiplicador * 10; 
            
            temporal = temporal / 2;
        }
        
        return resultado_binario; 
    }

    // 3. CONVERTIR A HEXADECIMAL
    fn a_hexadecimal(&self) -> String {
       
        return format!("{:X}", self.valor);
    }
}

fn main() {
    let mut mi_numero = Numero { valor: 45 };

    //binario
    let binario = mi_numero.a_binario();
    println!("El 45 en binario es: {}", binario); 
    // Salida: 101101

    //hexadecimal
    let hexa = mi_numero.a_hexadecimal();
    println!("El 45 en hexadecimal es: {}", hexa); 

//eliminar 
    mi_numero.eliminar_digito(4);
    println!("El 45 sin el 4 queda en: {}", mi_numero.valor); 
    // Salida: 5
}