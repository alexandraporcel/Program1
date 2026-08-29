struct Numero{
    valor: u64,
}
impl Numero { //contructor

    fn new(valor: u64) -> Self {
        Self { valor }
    }
    fn es_par(&self) -> bool {
        self.valor % 2 == 0
    }
    //Metodo para determinar si el valor de la instancia es : Mayor que un  numero dado por el usuario
    fn es_mayor_que(&self,x:&Numero) -> bool {
        self.valor > x.valor
    }
 //Metodo que devuelva la cantidad de digitos que tiene el valor de la instancia
    fn devolver_digitos(&self) -> u64 {
        let mut n = self.valor;
        let mut devolver = 0;

        if n == 0 {
            return 1;
        }

        while n > 0 {
            n /= 10;
            devolver += 1;
        }

        devolver
    }
 //Metodo que devuelva la cantidad de digitos impares que tiene el valor de la instancia
    fn devolver_digitos_impares(&self) -> u64 {
        let mut n = self.valor;
        let mut devolver = 0;

        if n == 0 {
            return 0;
        }

        while n > 0 {
            let digito = n % 10;
            if digito % 2 == 1 {
                devolver += 1;
            }
            n /= 10;
        }

        devolver
    }
//mETODO QUE DEVUELVA LA SUMA DE LOS NUMERO NATURALES QUE HAY en valor de la instancia ej: self.valor = 5: 1+2+3+4+5 = 15
fn suma_naturales(&self) -> u64 {
        let mut suma = 0;
        for i in 1..=self.valor {
            suma += i;
        }
        suma
    }

   
}
fn main() {
    println!("===================================");
    println!("Numeros");
    println!("===================================");
    let  n: Numero = Numero::new(4);
    println!("El valor de la instancia n es: {}", n.valor);
   println!("El valor es par? {}", n.es_par()); 
   let  x: Numero = Numero::new(5612);
   println!("El valor de n es mayor que el valor de x? {}", n.es_mayor_que(&x));
   println!("El numero de digitos en n es: {}", n.devolver_digitos());
   println!("El numero de digitos impares en n es: {}", n.devolver_digitos_impares());
   println!("El numero de digitos en x es: {}", x.devolver_digitos());
   println!("El numero de digitos impares en x es: {}", x.devolver_digitos_impares());
    println!("La suma de los numeros naturales hasta n es: {}", n.suma_naturales());
     println!("La suma de los numeros naturales hasta x es: {}", x.suma_naturales());
     
}
