pub fn main() {
    ejemplo_match2();
}

fn ejemplo_match2() {
    match (true, false) {
        (true, true) => print!("los 2 son true"),
        (_, false) => print!("solo el segundo es false"),
        _ => println!("ninguna de las anteriores"),
    };
}

fn ejemplo_match1() {
    let numero = 1;

    match numero {
        1 => println!("es 1"),
        2 => println!("es 2"),
        3 => println!("es 3"),
        4 | 5 => println!("4 o 5"),
        6..=10 => println!("está entre el 6 y el 10"),
        _ => println!("ninguno"),
    };

    let mensaje = match numero {
        1 => "es uno",
        _ => "es otro numero",
    };

    println!("{}", mensaje);
}

fn normales() {
    let color = "verde";
    if color == "verde" {
        println!("Verde");
    } else if color == "azul" {
        println!("Azul");
    } else {
        print!("ninguno");
    }

    println!("Terminó");
}
