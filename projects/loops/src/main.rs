fn main() {
    // infinite loop
    // loop {
    //     println!("Hello, world!");
    // }
    println!("**************************");


    // returning values from loops
    println!("Returning values from loops");
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    println!("**************************");

    // multiple loops
    println!("Multiple Loops");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!("**************************");

    // conditional loops with while
    println!("Conditional loop with while");
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("End number is {number}");

    println!("**************************");

    // looping through a collection with for
    println!("looping through a collection with for");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("{}", a[index]);
        index += 1;
    }
    println!("**************************");

    // looping through a collection with a better for approach
    println!("looping through a collection with for (better approach)");
    for element in a {
        println!("{element}");
    }

    println!("**************************");

    // loops with range
    println!("countdown with for and range");
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!");
    println!("**************************");



}
