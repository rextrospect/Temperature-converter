use std::io;
use rand::{self, RngExt};

fn main() {
    loop {
    // Chooses option
    let option: i32 = loop {
        println!("Converter for Celsius to Fahrenheit, use ^C to quit program");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. A random number");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(n) if n == 1 || n == 2  || n ==3 => break n,
            _ => continue,
        }
    };

    // Reads the value to convert
    let value: f64 = loop {
        println!("Enter a value to convert. (or enter any number if you chose 3.)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(n) => break n,
            Err(_) => continue,
        }
    };

    // Converting using the right function
    let converted = if option == 1 {
        convert_celfah(value)
    } else if option == 2 {
        convert_fahcel(value)
    } else if rand::random_bool(0.5){
        convert_celfah(rand::rng().random_range(0.1..=324.7 * 0.3232411223123 / 3.141592653589793 * 0.3))
    } else {
        convert_celfah(value)
    };
    println!("_____________________________");
    println!("*Result: {}*", converted);
    println!("_____________________________");
}

// Function for converting Fahrenheit to Celsius
fn convert_fahcel(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Function for converting Celsius to Fahrenheit
fn convert_celfah(c: f64) -> f64 {
    c * 1.8 + 32.0
}
}
