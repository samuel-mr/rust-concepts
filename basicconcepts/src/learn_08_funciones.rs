fn main() {
    demo_funciones_en_estructuras();
}

/* FUNCIONES  ************************/
fn demo_funciones() {
    let result = sumar(1, 2);
    println!("{}", result);

    let result = sumar2(1, 2);
    println!("{}", result);

    let result = factorial_clasico(5);
    println!("{}", result);

    let result = factorial_simplificado(5);
    println!("{}", result);
}
// Tradicional
fn sumar2(a: i32, b: i32) -> i32 {
    return a + b;
}

//Simplificado
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

// ejemplos
fn factorial_clasico(num: i32) -> i32 {
    if num == 1 {
        return num;
    }
    return num * factorial_clasico(num - 1);
}

// ejemplos
fn factorial_simplificado(num: i32) -> i32 {
    if num == 1 {
        num
    } else {
        num * factorial_simplificado(num - 1)
    }
}

/* FUNCIONES EN ESTRUCTURAS ************************/

fn demo_funciones_en_estructuras() {
    let mut usu = User {
        name: String::from("bach"),
    };
    usu.execGreeting();
    usu.name = "mozart".to_string();
    usu.execGreeting();
    usu.changeName("beethoven".to_string());
    usu.execGreeting();

    println!("{:?}", usu);
}

#[derive(Debug)]    // permite que se puede imprimir todos sus propiedades con println
struct User {
    name: String,
}
impl User {
    fn execGreeting(&self) {
        // self hace referencia a si mismo (como this)
        println!("Soy {}", self.name);
    }
    fn changeName(&mut self, newName: String) {
        // &mut -> hace referencia a si mismo y especifica que es modificable
        self.name = newName;
    }
}
