fn main() {
    let edad = 17;
    
    if edad >= 18 {
        println!("Ya podes votar... sos grandesito");
    } else {
        println!("Aún no podes votar... sos muy pelao");
    }


//LOOP
let mut intentos = 0;
loop {
intentos += 1;
println!("Ya llevas {} intentos ", intentos);
if intentos ==3 {
    println!("Cuenta bloqueada, demasiados intentos");
    break;
   }
  }
  //WHILE
  //Bateria
let mut bateria: i32 = 100;
while bateria >= 0 {
    println! ("Bateria restante: {}%", bateria);
    bateria -=20 ;
}
println!("Te quedaste sin bateria"); 

//FOR
    //CALCU POR 5
    
    for numero in 1..=10 {
        println!("5 x {} = {}", numero, numero * 5);
    }
//PARTE DE FUNCIONES

    let temp: f32 = 20.5; 
    let opcion = 2; 

    if opcion == 1 {
        println!("La temperatura de {} grados centígrados, es igual a {} grados fahrenheit.", temp, caf(temp));
    }

    if opcion == 2 {
        println!("La temperatura de {} grados fahrenheit, es igual a {} grados centígrados.", temp, fac(temp));
    }
}
//FUNCIONES CEL A FAR
fn caf(c: f32) -> f32 {
    (c * 1.8) + 32.0
}

fn fac(f: f32) -> f32 {
    (f - 32.0) / 1.8
}


