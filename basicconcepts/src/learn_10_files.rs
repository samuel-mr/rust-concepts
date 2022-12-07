use std::fs::{ File};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    read_file();
}

fn read_file() {
    let path = Path::new("assets/input.txt");
    let display = path.display();

    // Open (readonly)
    let mut file = match File::open(&path) {
        Err(why) => panic!("No se puede abrir {display}. Motivo: {why}"),
        Ok(file) => file,
    };

    // Read in string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("no puede leer {display}. Motivo: {why}"),
        Ok(file) => println!("leido. Size {file}!"),
    };
 
}