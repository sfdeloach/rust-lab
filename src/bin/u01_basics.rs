// read whitespace-separated integers from stdin, print min, max, sum, mean
// (as f64), and a line per value saying odd/even. Use if as an expression at
// least once, shadowing at least once, and a labeled break to stop at the
// first negative number.

use std::io;
fn main() {
    let mut integers = Vec::new();

    println!(
        "Enter non-negative integers separated by whitespace. A negative number stops processing."
    );

    // gather user input
    'lines: for line in io::stdin().lines() {
        let line = line.expect("read error");
        for token in line.split_whitespace() {
            let n: i64 = match token.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("skipping {token:?}");
                    continue;
                }
            };
            // exits both loops; a plain `break` would only exit the inner one
            if n < 0 {
                break 'lines;
            }
            integers.push(n);
        }
    }

    // find and print minimum
    let min = integers.iter().min().copied();

    match min {
        Some(value) => println!("min = {}", value),
        None => println!("There is no minimum."),
    }

    // find and print maximum
    let max = integers.iter().max().copied();

    match max {
        Some(value) => println!("max = {}", value),
        None => println!("There is no maximum."),
    }

    // calculate and print sum
    let sum: i64 = integers.iter().sum();
    println!("sum = {sum}");

    // calculate and print mean as f64
    match integers.len() {
        0 => println!("mean = 0"),
        _ => println!("mean = {}", sum as f64 / integers.len() as f64),
    }

    // line per value printing even/odd using if as an expression
    for integer in integers {
        println!(
            "{integer} is {}",
            if integer % 2 == 0 { "even" } else { "odd" }
        );
    }
}
