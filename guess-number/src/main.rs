use std::io;

fn main() {
    loop {
        let number: i32 = 7;

        println!("Hola, bienvenido a el juego de adivinar un numero\n");

        println!("Dime un numero del 1 al 10:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("No es un numero valido");

        let yournumber: i32 = input.trim().parse().expect("Ingrese un numero valido");

        if yournumber < number {
            println!("\nTu numero es menor, casi te acercas\n")
        } else if yournumber > number {
            println!("\nTu numero es mayor, casi te acercas\n")
        } else {
            println!("Perfecto el numero era {number}");
            break;
        }
    }
}
