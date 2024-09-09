use std::io;

fn main() {
    println!("Enter the desired number: ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read data from input");
    
    let number: u32 = input.trim().parse().expect("Unable to cast to unsigned number");

    println!("The {number}th Fibonacci number is: {}", fib(number));
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
