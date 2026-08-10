use std::io::stdin;

fn main() {
    println!("Game loss!");
    // This needs to be here so the terminal stays open
    let mut keep_program_open = String::new();
    stdin()
    .read_line(&mut keep_program_open)
    .expect("failed to read line");
}