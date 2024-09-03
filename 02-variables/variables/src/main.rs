fn main() {
    println!("Probando variables");
    let a: u8 = 25; // u18 significa unsigned 18 bits, solo numeros enteros sin signo o positivos;
    let b: i32 = 25; // i32 significa signed 32 bits, solo numeros enteros sin signo o positivos;
    let c: f32 = 25.7; // f32 significa float, solo numeros decimales;
    let d: f64 = 25.0; // f64 significa double, solo numeros decimales;
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("Hasta aca todo bien, interesante lo de los placeholder, 1: {}, 2: {}", c, a);

    let myName: &str = "Moises";
    println!("Hola soy {}! y estoy aprendiendo rust", myName);
    println!("Todas estas variables hasta ahora son inmutables, para hacer mutables usar let mut por ejemplo:");

    let mut apellido: &str = "Baldenegro";

    println!("Mi nombre es {} y mi apellido es {}", myName, apellido);

    apellido = "Melendez";

    println!("y ahora mi segundo apellido es {}, el cual fue impreso mutando la misma variable", apellido);
}
