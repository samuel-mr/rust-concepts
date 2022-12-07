// MCD = Máximo Común Divisor

#[test]
fn test_mcd() {
    assert_eq!(mcd_euclides_a(270, 192), 6);
}

pub fn mcd(a: i32, b: i32) -> i32 {
    let menor = if a < b { a } else { b };
    let mut i = menor;
    while i > 1 {
        println!("evaluando: {} {}", i, a % i);
        if a % i == 0 && b % i == 0 {
            return i;
        }
        i = i - 1;
    }
    1
}

pub fn mcd_euclides_a(a: i32, b: i32) -> i32 {
    let mayor = if a > b { a } else { b };
    let mut m = a;
    let mut n = b;
    while m != 0 && n != 0 {
        println!("m={} n={}", m, n);
        if m == 0 {
            return n;
        }
        if n == 0 {
            return m;
        }
        let res = m % n;
        m = n;
        n = res;
    }
    0
}
