use std::io;
use std::io::Write;

fn convert_to_fahrenheit(temp: f32) -> f32 {
    (temp * (9.0 / 5.0)) + 32.0
}

fn convert_to_celcius(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}

fn main() {
    print!("Enter a temp ending in c or f: ");
    io::stdout().flush().unwrap();
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    temp = temp.trim().to_lowercase().to_string();

    let unit = temp.pop();

    match unit {
        Some('c') => println!("{temp}c is {}f", convert_to_fahrenheit(temp.parse().unwrap())),
        Some('f') => println!("{temp}f is {}c", convert_to_celcius(temp.parse().unwrap())),
        _ => println!("{:?} is not a temperature ending in c or f!", temp),
    };
}
