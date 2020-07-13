fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameter(5);
    another_function_with_multiple_parameters(10,15);
    statement_and_expressions();

    let number : i32 = return_value();

    println!("The value of number is: {}", number);

    let doubled_number : i32 = return_value_with_parameter(10);

    println!("The value of doubled_number is: {}", doubled_number);

}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_with_multiple_parameters(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statement_and_expressions() {
    // statements. statement doesn't return value.
    let _x = 5;

    // expressions. that returns value because expression doesn't have semicolon at the end.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

// return type i32
fn return_value() -> i32 {
    5
}

fn return_value_with_parameter(t: i32) -> i32 {
    2 * t
}
