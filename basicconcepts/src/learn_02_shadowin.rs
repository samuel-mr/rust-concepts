pub fn main() {
    let x = 1;
    {
        println!("Inner: {}", x); // print 1

        // shadows: oculta la variable superior 'x'
        let x = "abc"; 

        println!("Inner: {}", x); // print 'abc'
    }
    // sin embargo x persiste en su contexto
    println!("Outer: {}", x); // print 1

    // shadows
    let x = true;
    println!("Outer: {}", x); // print true
}
