//Neta que como me mama hacer menus interactivos para modificar hasta las mas minima mierda en esto
//ouyea

use rand::RngExt;
use std::io::{self, Write};

#[derive(Debug)]
struct Datos {
    numero: i32,
    yournumber: i32,
    input: String,
}

fn validar_producto(datos: &Datos) -> Result<(), &'static str> {
    if datos.numero < 0 {
        return Err("El dato es menor al requerido; no puede ser negativo");
    }

    if datos.yournumber < 0 || datos.yournumber > 10 {
        return Err(
            "El dato ingresado no es compatible con el sistema; debe estar en un rango de 0 a 10",
        );
    }

    if datos.input.trim().is_empty() {
        return Err("No se ingreso ningun numero");
    }

    Ok(())
}

fn procesar_texto(datos: &mut Datos) -> Result<(), &'static str> {
    datos.input.clear();

    io::stdin()
        .read_line(&mut datos.input)
        .map_err(|_| "No se pude leer ese dato")?;

    datos.yournumber = datos
        .input
        .trim()
        .parse::<i32>()
        .map_err(|_| "No es valido el dato")?;

    Ok(())
}

fn randomizador(datos: &mut Datos) -> Result<(), &'static str> {
    datos.numero = rand::rng().random_range(1..=10);

    Ok(())
}

fn main() {
    println!("Bienvenido al guess-number-game de Mendozainc");

    let mut datos = Datos {
        numero: 0,
        yournumber: 0,
        input: String::new(),
    };

    randomizador(&mut datos).expect("No se pudo generar el numero");

    loop {
        print!("Ingrese un numero del 1 al 10: ");
        io::stdout().flush().expect("No se pudo mostrar el mensaje");

        if let Err(error) = procesar_texto(&mut datos) {
            println!("{error}");
            continue;
        }

        if let Err(error) = validar_producto(&datos) {
            println!("{error}");
            continue;
        }

        if datos.yournumber < datos.numero {
            println!("El numero secreto es mayor");
        } else if datos.yournumber > datos.numero {
            println!("El numero secreto es menor");
        } else {
            println!("Ganaste, el numero era {}", datos.numero);
            break;
        }
    }
}
