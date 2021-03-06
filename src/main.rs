extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    welcome_message();
    let mut board: [String; 9] = Default::default();
    let (player_letter, computer_letter) = choose_letter();
    println!("***********");
    println!("Player is {} and computer is {}", player_letter, computer_letter);

    // iterator to keep track of number of moves done
    let mut tot_turns = 0;

    // loop through asking the player for moves and also redrawing the board
    // idea: print out the current number of turns
    loop {
        // draw the board
        println!("Board is currently:");
        draw_board(&mut board);

        if player_letter == "X" {
            // ask user for a guess
            user_move(&mut board, "X".to_string());
            computer_move(&mut board, "O".to_string());
            draw_board(&mut board);

            if is_winner(&mut board, "X".to_string()) {
                println!("You win!!");
                break();
            } else if is_winner(&mut board, "O".to_string()){
                println!("Computer wins :(");
                break();
            }
        } else {
            user_move(&mut board, "O".to_string());
            computer_move(&mut board, "X".to_string());
            draw_board(&mut board);

            if is_winner(&mut board, "O".to_string()) {
                println!("You win!!");
                break();
            } else if is_winner(&mut board, "X".to_string()){
                println!("Computer wins :(");
                break();
            }
        }
        tot_turns = tot_turns + 1;
        println!("");
        if tot_turns == 9 {
            println!("It's a tie!");
        }
    }
    // todo: win condition
}

fn welcome_message() {
  println!("Welcome to tic-tac-toe!");
}

// return letters
fn return_letters(one: String, two: String) -> (String, String) {
    (one, two)
}

// check for a winner
fn is_winner(board: &mut [String], letter: String) -> bool {
    if board[0] == letter && board[1] == letter && board[2] == letter { // horizontal - top
        return true;
    } else if board[3] == letter && board[4] == letter && board[5] == letter { // horizontal - middle
        return true;
    } else if board[6] == letter && board[7] == letter && board[8] == letter { // horizontal - bottom
        return true;
    } else if board[0] == letter && board[3] == letter && board[6] == letter { // verical - left
        return true;
    } else if board[1] == letter && board[4] == letter && board[7] == letter { // verical - middle
        return true;
    } else if board[2] == letter && board[5] == letter && board[8] == letter { // verical - right
        return true;
    } else if board[0] == letter && board[4] == letter && board[8] == letter { // diagonal - top left to bottom right
        return true;
    } else if board[2] == letter && board[4] == letter && board[6] == letter { // diagonal - bottom left to top right
        return true;
    } else {
        return false;
    }
}

// checks if the letter is equal to X or O
fn letter_is_valid(letter: &str) -> bool {
    if letter.trim() == "X" || letter.trim() == "O" {
        return true;
    } else {
        return false;
    }
}

// returms tuple of assigned letters
fn assign_letters(letter: &str) -> (String, String) {
    if letter.trim() == "X" {
        return return_letters("X".to_string(), "O".to_string());
    } else {
        return return_letters("O".to_string(), "X".to_string());
    }
}

// lets the player choose a letter
// computer will be other letter
fn choose_letter() -> (String, String) {
    loop {
        let mut letter = String::new();

        println!("Choose a letter X or O: ");
        io::stdin().read_line(&mut letter)
            .ok()
            .expect("Failed to read line");

        let user_input = letter.to_uppercase();

        if letter_is_valid(&user_input) == false {
            println!("Please enter 'X' or 'O' :(");
        } else {
            return assign_letters(&user_input);
        }
    }
}

// draw a space if no value given
// o.w, printing looks a bit wonky
fn draw_board(board: &mut [String]) -> () {
  println!(" {} | {} | {}", board[0], board[1], board[2]);
  println!("-----------");
  println!(" {} | {} | {}", board[3], board[4], board[5]);
  println!("-----------");
  println!(" {} | {} | {}", board[6], board[7], board[8]);
}

// updates a place on the board with an 'X' or 'O'
fn update_board(board: &mut [String], i: usize, letter: String) -> () {
    board[i] = letter;
}

// checks for board input out of bounds
fn is_out_of_bounds( i: usize ) -> bool {
    // change logic to read a result instead
    if i>=9 {
        return true;
    } else {
        return false;
    }
}

// checks that the board space is empty
fn can_make_a_move(board: &mut [String], i: usize) -> bool {
    if is_out_of_bounds(i) {
        return false;
    } else if board[i] == "" {
        return true;
    } else {
        return false;
    }
}

fn user_move(board: &mut [String], user_letter: String) -> () {
    println!("Where would you like to move? Enter a value 1-9:");

    let mut user_input = String::new();

    loop {
        io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");

        let user_input: usize = user_input.trim().parse()
            .expect("Please type a number!");

        let move_space = user_input - 1;

        if can_make_a_move(board, move_space) {
            update_board(board, move_space, user_letter);
            break();
        } else {
            println!("Not a valid move! Enter a number for a free space!");
        }
    }
}

fn computer_move(board: &mut [String], computer_letter: String) {
    loop {
        let rand_number = rand::thread_rng().gen_range(0, 9);

        if can_make_a_move(board, rand_number) {
            println!("Computer draws a {} at space {}", computer_letter, rand_number+1);
            update_board(board, rand_number, computer_letter);
            break();
        } else {
            // do nothing?
        }
    }
}

