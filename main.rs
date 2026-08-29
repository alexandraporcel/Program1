struct Numero { 
    valor: u64
}

impl Numero {
    // Constructor
    fn new(valor: u64) -> Self { 
        Numero { valor }
    }
    
    fn es_par(&self) -> bool {
        self.valor % 2 == 0
    }
}

/// .
fn main() {
    println!("===================================");
    println!("Struct Numero");
    println!("===================================");
    
  
    let mut n: Numero = Numero::new(874);
    
    println!("El valor de la instancia n es: {}", n.valor);
    println!("N es par? {}", n.es_par());
}
