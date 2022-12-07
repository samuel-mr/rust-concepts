#![allow(dead_code)]
/********************* STRINGS *********************************/

pub fn cadenas() {
    // str -> cadena  inmutable
    let variable_str = "Soy Str";
    let mut variable_Sring = String::new();
    let mut variable_Sring2 = String::from("iniciado");

    variable_Sring2.push('.');
    variable_Sring2.push('.');
    variable_Sring2.push('.');
    variable_Sring2.push_str(" !!!");

    println!("{}", variable_Sring2);
}