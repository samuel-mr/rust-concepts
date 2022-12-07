mod exercise_mcd_1;
mod exercise_mcd_2;
// MCD = Máximo Común Divisor
fn main() {
    let result = exercise_mcd_1::mcd_euclides_a(270, 192);

    println!("MCD: {}", result);
}
