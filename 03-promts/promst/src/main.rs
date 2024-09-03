fn main() {
// Promts en rust para interacturar con el usuario

// creamos una variable de tipo string mutable para guardar el nombre del usuario
let mut nombre = String::new();

// imprimimos el mensaje de bienvenida
println!("Bienvenido a mi aplicacion");

// leemos el nombre del usuario
println!("Por favor ingrese su nombre:");

// leemos el nombre del usuario
std::io::stdin().read_line(&mut nombre).unwrap();

// imprimimos el nombre del usuario
println!("BIENVENIDO {}, Cual es su apellido?", nombre);

let mut apellido = String::new();
apellido.trim().to_string();

std::io::stdin().read_line(&mut apellido).expect("Error al leer el apellido");
println!("Bienvenido {},{}, increible que estes por aca aprendiendo rust", nombre, apellido);




}
