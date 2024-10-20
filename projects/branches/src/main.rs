fn main() {
    println!("Hello, world!");

    // if and else control flow
    let number = 33;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("Condition was false");
    }

    // if, else if and else control flow
    let number = 5;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    // since if is an expression, we can use it on the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

}
