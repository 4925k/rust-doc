fn main() {
    println!("Hello, world!");

    function();
    another_function(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");

}

// basic function
fn function() {
    println!("Another function.");
}

// function with an argument
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// function with multiple arguments
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// function with return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}