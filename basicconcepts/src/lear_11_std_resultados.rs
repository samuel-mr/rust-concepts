#![allow(dead_code)]

/********************* RESULT *********************************/
#[derive(Debug)]
enum DivisionErrorType {
    ByCero,
    ByNegative,
}
fn division2(a: i32, b: i32) -> Result<i32, DivisionErrorType> {
    if b == 0 {
        Err(DivisionErrorType::ByCero)
    } else if b < 0 {
        Err(DivisionErrorType::ByNegative)
    } else {
        Ok(a / b)
    }
}

fn result_exitoso() {
    let result = division2(3, 2);
    match result {
        Ok(1) => println!("El valor es uno"),
        Ok(2) => println!("El valor es dos"),
        Ok(valor) => println!("El valor es {}", valor),
        Err(DivisionErrorType::ByCero) => print!("No se puede dividir por 0"),
        Err(DivisionErrorType::ByNegative) => print!("No se puede dividir con negativos"),
    };
}

fn result_AsVariable() {
    let result: Result<i32, String> = Ok(2);
    let result: Result<i32, String> = Err(String::from("mensaje"));
}

fn result_unwrap_withError() {
    let resultado = division2(2, 0);
    let result = resultado.unwrap();
}

fn result_unwrap_withDefault() {
    let resultado = division2(2, 0);
    let result = resultado.unwrap_or(0);
    println!("{}", result);
}

fn result_unwrap_or_else() {

    fn obtenerCantidad(x: &str) -> usize{
        return x.len();
    }

    let result = Ok(2).unwrap_or_else(obtenerCantidad);
    println!("{}", result);
    let result = Err("foo").unwrap_or_else(obtenerCantidad);
    println!("{}", result);
}

fn result_unwrap_expect() {
    let resultado = division2(2, 0);
    let result = resultado.expect("Hay algun error en la división");
    println!("{}", result);
}
