use std::io;

pub fn main() {
    println!("Ingresa el nombre de usuario")
    ;
    let mut username = String::new();
    // read_line: retorna 'Result' q puede tener 2 valores, OK y error
    io::stdin().read_line(&mut username); 
    // aplico shadowin ya que username es &str, no un string
    let username = username.trim();
    

    println!("El valor ingresado es: {}",username);
}