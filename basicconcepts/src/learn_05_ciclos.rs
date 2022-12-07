pub fn main() {
    ejem_while();
}

fn ejem_while() {
    let mut i = 0;
    while i < 5 {
        println!("{}", i);
        i = i + 1;
    }
}

fn ejem_for() {
    println!("ejemplo a:");
    let numeros = [5, 4, 3, 2, 1];
    for item in numeros.iter() {
        println!("{}", item);
    }

    println!("ejemplo b:");
    for numero in 1..3 {
        println!("{}", numero);
    }
}

fn ejem_loop() {
    let mut i = 0;
    loop {
        println!("i = {}", i);
        if i == 10 {
            break;
        }
        i = i + 1;
    }
}
