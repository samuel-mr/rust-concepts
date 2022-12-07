pub fn main() {
    estructuras_tipo_tupla();
}
// enumeradores
fn enumeradores() {
    enum Response {
        Ok,
        Error(u32, String), //permite numeros enteros positivos
    }

    let result = Response::Error(501, String::from("ocurrió algun error !"));
    match result {
        Response::Ok => println!("OK !!!"),
        Response::Error(403, _) => println!("Forbidden"),
        Response::Error(404, _) => println!("Not found"),
        Response::Error(405, _) => println!("Internal Server Error"),
        Response::Error(_, mensaje) => println!("{}", mensaje),
    };
}

fn estructuras() {
    struct User {
        username: String,
        password: String,
    }

    let usuario = User {
        username: String::from("skynet"),
        password: String::from("pass"),
    };

    println!("usuario:{} password:{}", usuario.username, usuario.password);
}

fn estructuras2() {
    struct User {
        username: String,
        password: String,
    }

    let username = String::from("skynet");
    let password = String::from("pass");

    let mut us = User { username, password };
    us.password = String::from("passssss");

    println!("usuario:{} password:{}", us.username, us.password);
}

fn estructuras_tipo_tupla(){

    #[derive(Debug)]
    struct Color(u32, u32, u32);

    let white = Color(255,255,255);

    println!("El color es: {:?}", white);

}
