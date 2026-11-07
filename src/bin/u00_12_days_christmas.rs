use std::io::{self, Write};

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn fahr_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("Fahrenheit to Celsius");

    loop {
        print!("Enter a temp in deg F or 'x' to exit: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        temp = temp.trim().to_lowercase();

        if temp == "x" {
            break;
        }

        let temp: f64 = match temp.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number, try again.");
                continue;
            }
        };

        println!("{:.1} deg C", fahr_to_celsius(temp));
    }
}
