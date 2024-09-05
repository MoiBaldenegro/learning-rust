fn main() {
    println!("vamos a jugar un juego, encuentra el rango de numeros aceptados");
    println!("ingrese un numero");

    let mut counter:i8 = 0;

    loop {

       if counter >= 1 && counter <= 3 {
        println!("Intentalo una ves mas, ingresa otro numero");
       } else if counter == 3 {
        println!("Ya no te quedan mas intentos, no has acertado el rango correcto");
        break;
       }

        let mut num: String = String::new();
        std::io::stdin().read_line(&mut num).expect("Error al leer el numero");
        let num: u8 = num.trim().parse().expect("Error al parsear el numero");
        println!("tu numero es {}", num);

        if num > 100 {
            println!("tu numero es demasiado grande ");
            counter += 1;
        } else if num < 95 {
            println!("tu numero es demasiado pequeño");
            counter += 1;
        } else if num > 95 && num < 100 {
            println!("tu numero es entre 95 y 100, que es el rango correcto, FELICIDADES");
            break;
        }
    }
}
