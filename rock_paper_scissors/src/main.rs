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
    let player_won = get_player_win(&player_one_action, &player_two_action);
    if player_won == 0 {
        println!("Player one chose {player_one_action}, player two chose {player_two_action}, it was a tie!")
    }
    else if player_won == 23 {
        println!("Something broke! Uh oh! But player one chose {player_one_action}, player two chose {player_two_action}.")
    }
    else {
        println!("Player one chose {player_one_action}, player two chose {player_two_action}, player {player_won} wins!");
    }

    println!("Press enter to close the program.");
    io::stdin()
    .read_line(&mut String::new())
    .expect("meow");

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

fn get_player_win(player_one_action : &String, player_two_action : &String) -> u32 {
    let player_won: u32;
    if player_one_action == "rock" && player_two_action == "paper" {
        player_won = 2;
    }
    else if player_one_action == "rock" && player_two_action == "scissors" {
        player_won = 1;
    }
    else if player_one_action == "paper" && player_two_action == "rock" {
        player_won = 1;
    }
    else if player_one_action == "paper" && player_two_action == "scissors" {
        player_won = 2;
    }
    else if player_one_action == "scissors" && player_two_action == "rock" {
        player_won = 2;
    }
    else if player_one_action == "scissors" && player_two_action == "paper" {
        player_won = 1;
    }
    else if player_one_action == player_two_action {
        player_won = 0;
    }
    else {
        println!("Shit man idk who won this one");
        player_won = 23 // 23 to show that shit is fucked up
    }

    return player_won;
}