use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("generando numero!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("el numero secreto es: {secret_number}");
    loop {
       println!("ingrese un numero.");
       let mut guess = String::new(); //  variable para almacenar la entrada del usuario
       io::stdin()
         .read_line(&mut guess) // acceder a la referencia y volverla mutable 
         .expect("fallo al leer la linea"); // manejo de un posible error
       println!(" tu numero es: {guess}"); 
       match guess.cmp(&secret_number.to_string()) {
         Ordering::Less => println!("es mas pequeño!"),
         Ordering::Greater => println!("es mas grande!"),
         Ordering::Equal => println!("ganaste!"),
        }
    }

}

