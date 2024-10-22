fn main() {
    let n = 5;
    println!("Generating the {n}th fibonacci number!");

    let mut x = 0;
    let mut y = 1;
    for _ in 1..n {
        let temp = x;
        x += y;
        y = temp;
    }

    println!("The value of {n}th fibonacci number is {x}")
}
