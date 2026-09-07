struct Numero {
    valor: u64
}

impl Numero {
    //Constructor
    fn new(valor: u64) -> Self {
        Numero { valor }
    }

    fn es_par(&self) -> bool {
        self.valor % 2 == 0
    }

    //fn es_par(x: u64) -: bool 
    //     x % 2 == 0;

    //Multiplicar el valor (self) del objeto por el número dado.
    fn multiplicar(&self, num: u64) -> u64 {
        self.valor * num
    }
    //fn multiplicar(&self, num: u64) -....
    fn multiplicar(&self, num: u64) -> u64 {
        self.valor * num
    }
    //self.valor * num;
    fn es_mayor_que(&self, x: &Numero) -> bool {
        self.valor > x.valor
    }
}

fn main() {
    //funcion main
    println!("=============================");
    println!("Struct Numero");
    println!("=============================");
    //Crear instancia del objeto Numero: n
    let n = Numero::new(123);
    let x = Numero::new(456);
 
    println!("El valor de la instancia n es: {}", n.valor);

    println!("El valor n es par?: {}", n.es_par());
    println!("El valor n es mayor que el valor de x?: {}", n.es_mayor_que(&x));
    println!("El resultado de multiplicar n por 3 es: {}", n.multiplicar(3));

    //println!("El valor n es par?: {}", n.multiplicar(3));
}