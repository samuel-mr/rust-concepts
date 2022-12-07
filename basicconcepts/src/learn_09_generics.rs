// ME QUEDE AQUI: https://doc.rust-lang.org/book/ch10-01-syntax.html#in-struct-definitions

use std::iter::Copied;

fn main() {
    let result = search_greatest(&[3, 9, 5, 6, 7, 2]);
    println!("{result}");
}

fn search_greatest<T>(num: &[T]) -> T
where
    T: std::cmp::PartialOrd + std::marker::Copy
{
    let mut greatest = &num[0];
    for i in num {
        if i > greatest {
            greatest = i;
        }
    }
    *greatest
}
fn search_greatest_version3<T: std::cmp::PartialOrd + std::marker::Copy>(num: &[T]) -> T {
    let mut greatest = &num[0];
    for i in num {
        if i > greatest {
            greatest = i;
        }
    }
    *greatest
}

// esta version solo cambia los tipos de datos donde ahora no uso punteros
fn search_greatest_version2<T: std::cmp::PartialOrd + std::marker::Copy>(num: &[T]) -> T {
    let mut greatest = num[0];
    for &i in num {
        if i > greatest {
            greatest = i;
        }
    }
    greatest
}
