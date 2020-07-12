fn main() {

    // Rust performs two’s complement wrapping. ref: https://en.wikipedia.org/wiki/Two%27s_complement
    // Signed integers starts with i and can be negative and positive. -127 to 127
    let i : i8 = 127;

    let d : i8 = 12_5;

    println!("The value of d is: {}", d);

    // Unsigned integers with u and can be only positive. 0 to 255
    let u: u8 = 255;
    
    // Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.
    let n = 2.0; // By default f64.

    let f : f32 = 3.0; // Even it is f32 is smaller than f64 they have same speed.

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation


    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
    let c = 'z';
    let z : char = 'ℤ';
    let heart_eyed_cat = '😻';

    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let tupple: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tupple.0;

    let six_point_four = tupple.1;

    let one = tupple.2;

    println!("The value of y five_hundred: {}", five_hundred);

    // Arrays are useful when you want your data allocated on the stack rather than the heap.
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];

    let arry: [i32; 5] = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // Both arrays are same [3, 3, 3, 3, 3].
    let shorter = [3; 5];
    let longer = [3, 3, 3, 3, 3];          

}
