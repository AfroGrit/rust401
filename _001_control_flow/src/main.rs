fn five() -> i32 {
    // this function returns 5
    5
}

fn another_function() {
    // this is pseudo function
    println!("pseudo function.");
}

fn main() {
    println!("1. functions.");
    another_function();
    let x = five();
    println!("2. return functions: {x}");

    // control flow
    if x < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = false;
    let mut number = if condition { 2 } else { 3 };
    println!("The value of number is: {number}");

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    
    for number in (1..8).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

