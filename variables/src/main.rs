fn main() {

    // immutable
    /* by default variables are immutable in Rust so once you
    assigned, you can't reassign.
        let x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    */

    // mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constant
    // const MAX_POINTS: u32 = 100_000;

    // shadowing is different from mut because you can assign a new data type like spaces example.
    let t = 5;

    let t = t + 1;

    let t = t * 2;

    println!("The value of x is: {}", t);

    /* shadowing different data types example.
        let spaces = "   ";
        let spaces = spaces.len();
    */
}
