fn main() {

    
    println!("vamos a jugar un juego, encuentra el rango de numeros aceptados");
    println!("ingrese un numero");
    let mut num: String = String::new();
    std::io::stdin().read_line(&mut num).expect("Error al leer el numero");
    let num: u8 = num.trim().parse().expect("Error al parsear el numero");
    println!("tu numero es {}", num);

    if num > 100 {
        println!("tu numero es demasiado grande");
    } else if num < 95 {
        println!("tu numero es demasiado pequeño");
    } else if num > 95 && num < 100 {
        println!("tu numero es entre 95 y 100, que es el rango correcto, FELICIDADES");
    }
}
