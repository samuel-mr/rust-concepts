
fn main() {
    bloques();
}

fn bloques() {
    let resultado = {
        let variable: i32 = 10;
        variable //la última línea quiere decir que de esa variable se retornará su VALOR
    };
    println!("{}", resultado);

    let mensaje = if resultado == 10 {
        String::from("El resultado es 10")
    } else {
        String::from("El resultado no es 10")
    };

    println!("{}", mensaje);
}

fn declaracion_acceso() {
    // Asignación de tipo
    let implicito = 2;
    let explicito: i32 = 2;

    // Tipos de acceso
    // let     => inmutable
    // let mut => mutable

    let numero1 = 10;
    let mut numero2 = 2;
    const PI: f64 = 3.14;

    // Notación
    let an_integer1: i32 = 5; // tradicional
    let an_integer2 = 5i32; // sufijo
    let an_integer3 = 5; // inferido

    // Notación: inferido por contexto
    let mut inferido_en_contexto = 2; // i64 será el tipo. Inferido por la siguiente línea
    inferido_en_contexto = 2i64;

    println!("1: {}", an_integer1);
    println!("2: {}", an_integer2);
    numero2 = 33;
    let resultado = numero1 * numero2;

    println!(
        "El resultado es : {} + {} = {}",
        numero1, numero2, resultado
    );
}


fn operadores() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}