fn main() {
    // sin variable

    const VARIABLE_ONE: i32 = 25;
    const VARIABLE_TWO: i32 = 50;

    match VARIABLE_ONE {
        25 => println!("Es el 25"),
        0..=25 => println!("Es entre 0 y 25"),
        26..50 => println!("Es entre 26 y 50"),
        _ => println!("something else"),
    }

    //con variable
    let message = match VARIABLE_TWO {
        25 => "Es el 25",
        0..=24 => "Es entre 0 y 25",
        26..=50 => "Es entre 26 y 50",
        _ => "something else",
    };

    println!("MESSAGE: {}", message);
}
