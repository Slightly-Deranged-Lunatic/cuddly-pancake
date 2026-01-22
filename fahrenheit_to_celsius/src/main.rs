use std::io;

fn main() {
    let mut fahrenheit_temperature = String::new();

    let fahrenheit_temperature = loop {
        println!("Please input the fahrenheit temperature: ");
        io::stdin()
            .read_line(&mut fahrenheit_temperature)
            .expect("Failed to read line");

        let fahrenheit_temperature: f64 = match fahrenheit_temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        break fahrenheit_temperature;
    };
    let celsius_temperature = (fahrenheit_temperature - 32.0) /1.8;
    println!("{fahrenheit_temperature} fahrenheit to celsius is {celsius_temperature}.");
}

