use std::io;

fn main() {
    let operation = get_operation();
    let number_1 = get_number();
    let number_2 = get_number();
    clearscreen::clear().expect("failed to clear screen");

    if operation == "addition" {
        let result = number_1 + number_2;
        println!("{number_1} plus {number_2} is {result}.");
    }
    else if operation == "subtraction" {
        let result = number_1 - number_2;
        println!("{number_1} minus {number_2} is {result}.");
    }
    else if operation == "multiplication" {
        let result = number_1 * number_2;
        println!("{number_1} times {number_2} is {result}.");
    }
    else if operation == "division" {
        let result = number_1 / number_2;
        println!("{number_1} divided {number_2} is {result}.");
    }

    println!("Press enter to close the program.");
    io::stdin()
    .read_line(&mut String::new())
    .expect("meow");
}

fn get_operation() -> String {
    static VALID_OPERATIONS : [&str; 4] = ["addition", "subtraction", "multiplication", "division"];
    let mut operation = String::new();

    loop {
        println!("Please input an operation, addition, subtraction, multiplication, or division.");

        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line!");

        operation = operation.trim().to_lowercase();
        if ! VALID_OPERATIONS.contains(&&operation.as_str()) {
            println!("Invalid operation!");
        }
        else {
            return operation;
        }
    }
}

fn get_number() -> f64 {
    let mut number: String = String::new();

    loop {
        println!("Please input a number: ");

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line!");

        let number:f64 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };
        return number;
    }
}