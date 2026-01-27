use std::io;

fn main() {
    println!("Welcome to rock paper scissors");
    println!("Player 1 please enter your move");
    static VALID_INPUT : [&str; 3] = ["rock", "paper", "scissors"];
    let player_one_action = get_player_action(VALID_INPUT);
    clearscreen::clear().expect("failed to clear screen");
    println!("Player 2 please enter your move");
    let player_two_action = get_player_action(VALID_INPUT);
    clearscreen::clear().expect("failed to clear screen");
    }

fn get_player_action(VALID_INPUT: [&str; 3]) -> String {
    loop {
        let mut player_action = String::new();
        io::stdin()
            .read_line(&mut player_action)
            .expect("Failed to read line!");
        player_action = player_action.trim().to_lowercase();
        if ! VALID_INPUT.contains(&&player_action.as_str()) {
            println!("Invalid input");
        }
        else {
            return player_action
        }
    }
}
