struct Numero {
    valor: u64,
}

impl Numero {
    // 1. Devuelve la cantidad de números que son divisores (múltiplos) sin tomar en cuenta el valor
    fn multiplo(&self) -> u64 {
        let mut contador = 0;
        for i in 1..self.valor {
            if self.valor % i == 0 { 
                contador += 1;
            }
        }
        contador
    }

    // 2. Invertir los dígitos del número
    fn invertir_digitos(&self) -> u64 {
        let mut numero = self.valor; 
        let mut invertido = 0;
        
        while numero > 0 {
            invertido = (invertido * 10) + (numero % 10);
            numero /= 10;
        }
        invertido
    }

    // 3. Es capicua? 
    fn capicua(&self) -> bool {
        self.valor == self.invertir_digitos()
    }

    // 4. Calcula la raíz digital sumando los dígitos hasta que quede uno solo
    fn raiz_digital(&self) -> u64 {
        let mut numero = self.valor;
        
        while numero >= 10 {
            let mut suma = 0;
            let mut temporal = numero;
            
            while temporal > 0 {
                suma += temporal % 10;
                temporal /= 10;
            }
            numero = suma;
        }
        numero
    }
fn adicionar(&mut self, num: u64) {
    self.valor = (self.valor * 10) + num;
}
}

fn main() {
    
    let mut numero = Numero { valor: 60 };
      numero.adicionar(165);
    
    println!("Número: {}", numero.valor);
    println!("1. Cantidad de múltiplos (divisores): {}", numero.multiplo());
    println!("2. Número invertido: {}", numero.invertir_digitos());
    println!("3. Es capicúa?: {}", numero.capicua());
    println!("4. Raíz digital: {}", numero.raiz_digital());
    println!("5. Número adicional: {}", numero.valor);
}