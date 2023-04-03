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
        if check_for_win(&board, 'X') {
            print_board(&board);
            println!("You won!");
            break;
        }

        cpu_input(&mut board, 'O');
        if check_for_win(&board, 'O') {
            print_board(&board);
            println!("The computer won!");
            break;
        }
    }
}

fn user_input(board: &mut Vec<Vec<char>>, char: char) {
    print!("Which column would you like to play in? ");
    _ = io::stdout().flush();

    let mut user_col = String::new();
    io::stdin()
        .read_line(&mut user_col)
        .expect("Failed to read line");

    let user_col: usize = match user_col.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a number in the correct range.");
            user_input(board, char);
            return;
        }
    };

    if user_col < 1 || user_col > board[0].len() {
        println!("Please input a number in the correct range.");
        user_input(board, char);
        return;
    }

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

fn check_for_win(board: &Vec<Vec<char>>, char: char) -> bool {
    // check for horizontal win conditions
    for row in 0..board.len() {
        for col in 0..=(board[row].len() - 4) {
            if board[row][col] == char
                && board[row][col + 1] == char
                && board[row][col + 2] == char
                && board[row][col + 3] == char
            {
                return true;
            }
        }
    }

    // check for vertical win conditions
    for row in 0..=(board.len() - 4) {
        for col in 0..board[row].len() {
            if board[row][col] == char
                && board[row + 1][col] == char
                && board[row + 2][col] == char
                && board[row + 3][col] == char
            {
                return true;
            }
        }
    }

    // check for diagonal win conditions that look like \
    for row in 0..=(board.len() - 4) {
        for col in 0..=(board[row].len() - 4) {
            if board[row][col] == char
                && board[row + 1][col + 1] == char
                && board[row + 2][col + 2] == char
                && board[row + 3][col + 3] == char
            {
                return true;
            }
        }
    }

    // check for diagonal win conditions that look like /
    for row in 0..=(board.len() - 4) {
        for col in 3..board[row].len() {
            if board[row][col] == char
                && board[row + 1][col - 1] == char
                && board[row + 2][col - 2] == char
                && board[row + 3][col - 3] == char
            {
                return true;
            }
        }
    }

    return false;
}
