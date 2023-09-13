use std::io;

fn main() {
    println!("Fibonacci Sequence Calculator");

    let mut fib = String::new();
    println!("Input the N number below:");

    io::stdin()
        .read_line(&mut fib)
        .expect("failed to read number");

    let parsed_fib = fib.trim().parse();

    match parsed_fib {
        Ok(fibo) => {
            let result = fibonacci(fibo);
            println!("Fibonacci({}), = {}", fibo, result);
        }
        Err(_) => println!("erro"),
    };
}

fn fibonacci(num: i32) -> i32 {
    if num <= 1 {
        num
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}
