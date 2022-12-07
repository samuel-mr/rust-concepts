use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    // El 1er elemento es el nombre del probrama, por eso lo evitamos
    for arg in env::args().skip(1) {
        let nom = u64::from_str(&arg);
        // si from_str no es OK, lanzará un panic con el mensaje de error (al final)
        numbers.push(u64::from_str(&arg).expect("error al parsear el argumento"));
    }

    if numbers.len() == 0 {
        eprintln!("Usarlo así: cargo run 22 33"); //muestra un mensaje standard de error
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // &: le decimos a Rust que el propietario de 'numbers' permanecerá...
    //    y que en el for solo se enviará una referencia de cada valor de 'numbers'
    for m in &numbers[1..] {
        println!("x: {}", m);
        // *m : ya que m es una referencia, con el * obtiene el valor en si
        d = mcd_euclides_b(d, *m);
    }

    println!("MCD de {:?} es {}", numbers, d);
}

#[test]
fn test_mcd() {
    assert_eq!(mcd_euclides_b(270, 192), 6);
}
// Optimizado respecto a la versión anterior
fn mcd_euclides_b(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while a > 0 {
        if b > a {
            let temp = b;
            b = a;
            a = temp;
        }
        a = a % b;
    }
    b
}
