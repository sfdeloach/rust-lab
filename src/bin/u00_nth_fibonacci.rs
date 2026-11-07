use std::io::{self, Write};

/// Generate the nth Fibonacci number recursively
fn fib_recursive(n: u16) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

/// Generate the nth Fibonacci number iteratively
fn fib_iterative(n: u16) -> u128 {
    if n < 2 {
        return n.into();
    }

    let (mut n_2, mut n_1) = (0, 1);

    for _ in 2..n {
        (n_2, n_1) = (n_1, n_1 + n_2)
    }

    n_2 + n_1
}

fn main() {
    println!("nth Fibonacci number");

    print!("Enter n: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u16 = n.trim().parse().expect("Not a valid number.");

    println!("recursive: {}", fib_recursive(n));
    println!("iterative: {}", fib_iterative(n));
}
