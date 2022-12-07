#![allow(dead_code)]

/********************* OPTION *********************************/
fn dividir(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn opciones() {
    let result = dividir(3, 1);
    match result {
        Some(1) => println!("el valor es 1"),
        Some(2) => println!("el valor es 2"),
        Some(valor) => println!("{}", valor),
        None => println!("No se puede dividir entre 0"),
    }
}

fn opciones_unwrap_withError() {
    let variable_result = dividir(2, 0);
    let result = variable_result.unwrap();
}

fn opciones_unwrap_withDefault() {
    let variable_result = dividir(2, 0);
    let result = variable_result.unwrap_or(-1);
    println!("{}", result);
}

fn opciones_unwrap_expect() {
    let variable_result = dividir(2, 0);
    let result = variable_result.expect("Hay algun error en la división");
    println!("{}", result);
}

fn opciones_AsVariable() {
    let variable: Option<i32> = Some(32);
    let variable: Option<i32> = None;
}

fn opciones_InStruct() {
    #[derive(Debug)]
    struct User {
        nombre: String,
        edad: Option<i32>,
    };
    let entidad = User {
        nombre: String::from("bach"),
        edad: None,
    };

    println!("{:?}", entidad);
}