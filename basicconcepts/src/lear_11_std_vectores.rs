use std::fmt::{Debug, Display};

/********************* BASICO  *********************************/
pub fn vectores() {
    let mut vector_sindefinir = Vec::new();
    vector_sindefinir.push(1); //aqui se define el tipo de variable soportado del vector
    vector_sindefinir.push(2); //aqui se define el tipo de variable soportado del vector
    println!("{:?}", vector_sindefinir);

    let vector = vec![1, 2, 3];
    println!("declaracion implícita: {:?}", vector);

    let vector: Vec<i32> = vec![1, 2, 3];
    println!("declaracion explícita con valores: {:?}", vector);

    let mut vector: Vec<i32> = Vec::new();
    vector.push(2);
    println!("declaracion explícita: {:?}", vector);

    let mut vector = vec![9, 7, 5, 3];
    vector.push(1);
    vector.remove(0);

    vector.insert(0, -1);
    let last = vector.pop().unwrap();
    println!("elemento eliminado: {}", last);

    println!("{:?}", vector);
}

/********************* IMPLEMENTAR INTERFAZ EN VEC<T>  *********************************/
trait EnrichText {
    fn print_as_enumerable_list(&self);
}

impl<T> EnrichText for Vec<T>
where
    T: Display, // especifico que tendrá implementada esta intefaz, para que me permita hacer => {item}
{
    fn print_as_enumerable_list(&self) {
        for (pos, item) in self.iter().enumerate() {
            println!("[{pos}]: {item}");
        }
    }
}
pub fn implementando_interfaces() {
    let vec = vec!['a', 'b', 'c'];
    vec.print_as_enumerable_list();
}

/*---------------------------------------------------- */
// Ese es solo un ejemplo ya que por defecto Rust si podrá comparar un vector compuesto

trait ISuperComparable {
    fn are_equals(&self, other: &Self) -> bool;
}

impl<T> ISuperComparable for Vec<Vec<T>>
where
    T: Display + PartialEq,
{
    fn are_equals(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            let first = &self[i];
            let secound = &other[i];
            if   first ==  secound  { // Aun hay un error, esto no compara bien
                return false;
            }
        }
        return true;
    }
}

#[test]
pub fn comprar_vectores_de_vectores_test() {
    let source: Vec<Vec<char>> = vec![vec!['A'], vec!['X']];
    let result: Vec<Vec<char>> = vec![vec!['A'], vec!['X']];

  //  assert_eq!(result, source);
    assert!(source.are_equals(&result));
}

// use std::cmp::PartialEq;

// pub trait Comparable {
//     fn compare(&self, other: &Self) -> bool;
// }

// impl<T> Comparable for Vec<Vec<T>>
// where
//     T: PartialEq,
// {
//     fn compare(&self, other: &Self) -> bool {
//         self.len() == other.len() && self.iter().zip(other).all(|(a, b)| a == b)
//     }
// }
