fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let second_number = 6;

    if second_number % 4 == 0 {
        println!("second number is divisible by 4");
    } else if second_number % 3 == 0 {
        println!("second number is divisible by 3");
    } else if second_number % 2 == 0 {
        println!("second number is divisible by 2");
    } else {
        println!("second number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let third_number = if condition { 5 } else { 6 };

    println!("the value of third number is: {}", third_number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {}", result);


    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", array[index]);

        index += 1;
    }

    let second_array = [10, 20, 30, 40, 50];
    
    for element in second_array.iter() {
        println!("the value is: {}", element)
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

}
