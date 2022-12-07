use std::mem;
use std::any::type_name;


fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn tipos(){

    println!("Imprimir directamente el tipo --------------------");
    println!("Option<String> = {}",std::any::type_name::<Option<String>>());
    println!("String         = {}",std::any::type_name::<String>());
    println!("i32            = {}",std::any::type_name::<i32>());

    
    println!("");
    println!("Imprimir el tipo desde las variables --------------------");

    // Tipos
    let mut texto1 = "string";
    let mut texto2: &str = "string 2";

    println!("a = {}", type_of(texto1));
    println!("b = {}", type_of(&&&texto1));
    println!("c = {}", type_of(texto2));
    println!("d = {}", type_of(&texto2));

    let texto2 = String::from("String");
    //let ss = texto2.as_str();

     print_type_of(&texto2);
    //println!("B.. {tipo_variable} tipo:{texto2}");
}

// fn conversiones() {
//     println!("Conversiones ---------");
//     let texto = "5";
//     let numero: i32 = texto.parse().unwrap();

//     let numeroAsString = (numero + 1).to_string();
//     println!(numeroAsString);
// }
