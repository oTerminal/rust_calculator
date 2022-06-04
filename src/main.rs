use std::io;
use std::io::{Write};

fn main() {
    println!("Calculator Starting");
    
    println!("Supported Operations -> + is addition, - is subtraction, * is multiplication and / is division.");

    print!("Enter the operation you want to use: ");
    let mut input_operation = String::new();
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input_operation).expect("Unable to read line.");
    let input_operation: char = input_operation.trim().parse().unwrap();

    if input_operation == '+' {
        add_numbers()

    } else if input_operation == '-' {
        subtract_numbers()
    } else if input_operation == '*' {
        multiply_numbers()
    } else if input_operation == '/' {
        divide_numbers()
    }
}

fn add_numbers() {
    let mut anumber1 = String::new();
    print!("Enter your first number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut anumber1).expect("Unable to read line.");

    let anumber1: f32 = anumber1.trim().parse().unwrap();

    let mut anumber2 = String::new();
    print!("Enter your second number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut anumber2).expect("Unable to read line.");

    let anumber2: f32 = anumber2.trim().parse().unwrap();

    println!("The sum is {}", anumber1 + anumber2);
}

fn subtract_numbers() {
    let mut anumber1 = String::new();
    print!("Enter your first number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut anumber1).expect("Unable to read line.");

    let anumber1: f32 = anumber1.trim().parse().unwrap();

    let mut anumber2 = String::new();
    print!("Enter your second number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut anumber2).expect("Unable to read line.");

    let anumber2: f32 = anumber2.trim().parse().unwrap();

    println!("The difference is {}", anumber1 - anumber2);
}

fn multiply_numbers() {
    let mut anumber1 = String::new();
    print!("Enter your first number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut anumber1).expect("Unable to read line.");

    let anumber1: f32 = anumber1.trim().parse().unwrap();

    let mut anumber2 = String::new();
    print!("Enter your second number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut anumber2).expect("Unable to read line.");

    let anumber2: f32 = anumber2.trim().parse().unwrap();

    println!("The product is {}", anumber1 * anumber2);
}

fn divide_numbers() {
    let mut anumber1 = String::new();
    print!("Enter your first number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut anumber1).expect("Unable to read line.");

    let anumber1: f32 = anumber1.trim().parse().unwrap();

    let mut anumber2 = String::new();
    print!("Enter your second number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut anumber2).expect("Unable to read line.");

    let anumber2: f32 = anumber2.trim().parse().unwrap();

    println!("The quotient is {}", anumber1 / anumber2);
}