use rand::Rng;
use std::io::{self, Write};

fn main() {
    const COLUMNS: usize = 7;
    const ROWS: usize = 6;

    println!("Welcome to Connect Four!");

    let mut board = vec![vec![' '; COLUMNS]; ROWS];

    loop {
        print_board(&board);
        user_input(&mut board, 'X');
        // check_for_win('X');
        cpu_input(&mut board, 'O');
        // check_for_win('O');
    }
}

fn user_input(board: &mut Vec<Vec<char>>, char: char) {
    print!("Which column would you like to play in? ");
    _ = io::stdout().flush();

    let mut user_col = String::new();
    io::stdin()
        .read_line(&mut user_col)
        .expect("Failed to read line");

    let user_col: usize = user_col
        .trim()
        .parse()
        .expect("Failed to convert input to number");

    // iterate through the board, bottom to top
    for row in board.iter_mut().rev() {
        if row[user_col - 1] == ' ' {
            row[user_col - 1] = char;
            break;
        }
    }
}

fn print_board(board: &Vec<Vec<char>>) {
    // print column labels
    for num in 1..=(board[0].len()) {
        print!("  {num} ")
    }
    println!();

    // print top border
    for _ in 1..=(board[0].len()) {
        print!(" ___")
    }
    println!();

    // print actual pieces themselves
    for row in board.iter() {
        print!("|");
        for col in row.iter() {
            print!(" {col} |");
        }
        println!();
    }

    // print bottom border
    for _ in 1..=(board[0].len()) {
        print!(" ‾‾‾")
    }
    println!();
}

fn cpu_input(board: &mut Vec<Vec<char>>, char: char) {
    let cpu_col = rand::thread_rng().gen_range(0..board[0].len());

    // iterate through the board, bottom to top
    for row in board.iter_mut().rev() {
        if row[cpu_col] == ' ' {
            row[cpu_col] = char;
            break;
        }
    }
}
