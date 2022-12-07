fn main() {
    println!("Hello, world!");
}

fn arreglos() {
    let numeros = [5, 4, 3, 2, 1];

    println!("Los números son: {:?}", numeros);

    println!("first item: {}", numeros[0]);
    println!("longitud  : {}", numeros.len());

    println!("Ocupa: {} bytes", mem::size_of_val(&numeros));

    println!("Enviado como referencia TODO el arreglo:");
    arreglos_metodo(&numeros);

    println!("Enviado como referencia UNA PARTE del arreglo");
    arreglos_metodo(&numeros[1..3]);
}

fn arreglos_metodo(slice: &[i32]) {
    println!("(in method) longitud: {}", slice.len());
    println!("(in method) items   : {:?}", slice);
}

fn slices() {
    let mensaje = String::from("hola mundo");

    let completo = &mensaje[..];          // todo
    let inicio = &mensaje[0..4];          // indice incial .. indice final
    let inicio = &mensaje[..4];           // idem
    let fin = &mensaje[4..mensaje.len()]; // inicia en 4 hasta el final
    let fin = &mensaje[4..];              // idem

    println!("{}", inicio);
    println!("{}", fin);
}
fn tuplas() {
    let tupla = (1, true, 3.4);
    let tupla: (i32, bool, f64) = (1, true, 3.4);

    println!("tupla: {:?}", tupla);
    println!("first: {}", tupla.0);
    println!("last : {}", tupla.2);

    let tupla_anidada: ((i32, bool), (f64, i32));

    let tupla = (1, true);
    let (numero, boleano) = tupla;
    println!("numero:{}  boleano:{}", numero, boleano);
}
