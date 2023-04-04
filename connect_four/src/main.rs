use rand::Rng;
use std::io::{self, Write};

fn main() {
    const COLUMNS: usize = 7;
    const ROWS: usize = 6;

    println!();
    println!("Welcome to Connect Four!");

    let gamemode = prompt_for_gamemode();

    let mut board = vec![vec![' '; COLUMNS]; ROWS];

    if gamemode == 1 {
        println!();
        println!("Player 1 will be X and Player 2 will be O.");
        println!("Player 1 will go first.");

        loop {
            print_board(&board);

            print!("Player 1, ");
            user_input(&mut board, 'X');
            if check_for_win(&board, 'X') {
                print_board(&board);
                println!("Player 1 won!");
                break;
            }

            print_board(&board);

            print!("Player 2, ");
            user_input(&mut board, 'O');
            if check_for_win(&board, 'O') {
                print_board(&board);
                println!("Player 2 won!");
                break;
            }
        }
    } else {
        println!();
        println!("You will be X and the computer will be O.");
        println!("You will go first.");

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
}

fn user_input(board: &mut Vec<Vec<char>>, player: char) {
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
            user_input(board, player);
            return;
        }
    };

    if user_col < 1 || user_col > board[0].len() {
        println!("Please input a number in the correct range.");
        user_input(board, player);
        return;
    }

    // iterate through the board, bottom to top
    for row in board.iter_mut().rev() {
        if row[user_col - 1] == ' ' {
            row[user_col - 1] = player;
            return;
        }
    }

    // try again if there are no spaces left in the column
    println!("That column seems to be full.");
    user_input(board, player);
}

fn print_board(board: &Vec<Vec<char>>) {
    println!();

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

fn cpu_input(board: &mut Vec<Vec<char>>, player: char) {
    let cpu_col = rand::thread_rng().gen_range(0..board[0].len());

    // iterate through the board, bottom to top
    for row in board.iter_mut().rev() {
        if row[cpu_col] == ' ' {
            row[cpu_col] = player;
            println!("The computer played in column {}", cpu_col + 1);
            return;
        }
    }

    // try again if there are no spaces left in the column
    cpu_input(board, player);
}

fn check_for_win(board: &Vec<Vec<char>>, player: char) -> bool {
    // check for horizontal win conditions
    for row in 0..board.len() {
        for col in 0..=(board[row].len() - 4) {
            if board[row][col] == player
                && board[row][col + 1] == player
                && board[row][col + 2] == player
                && board[row][col + 3] == player
            {
                return true;
            }
        }
    }

    // check for vertical win conditions
    for row in 0..=(board.len() - 4) {
        for col in 0..board[row].len() {
            if board[row][col] == player
                && board[row + 1][col] == player
                && board[row + 2][col] == player
                && board[row + 3][col] == player
            {
                return true;
            }
        }
    }

    // check for diagonal win conditions that look like \
    for row in 0..=(board.len() - 4) {
        for col in 0..=(board[row].len() - 4) {
            if board[row][col] == player
                && board[row + 1][col + 1] == player
                && board[row + 2][col + 2] == player
                && board[row + 3][col + 3] == player
            {
                return true;
            }
        }
    }

    // check for diagonal win conditions that look like /
    for row in 0..=(board.len() - 4) {
        for col in 3..board[row].len() {
            if board[row][col] == player
                && board[row + 1][col - 1] == player
                && board[row + 2][col - 2] == player
                && board[row + 3][col - 3] == player
            {
                return true;
            }
        }
    }

    return false;
}

fn prompt_for_gamemode() -> u8 {
    println!();
    println!("  1. Player vs. Player");
    println!("  2. Player vs. Computer");
    println!();
    print!("What gamemode would you like to play? ");
    _ = io::stdout().flush();

    let mut gamemode = String::new();
    io::stdin()
        .read_line(&mut gamemode)
        .expect("Failed to read line");

    let gamemode: u8 = match gamemode.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a number in the correct range.");
            return prompt_for_gamemode();
        }
    };

    if gamemode < 1 || gamemode > 2 {
        println!("Please input a number in the correct range.");
        return prompt_for_gamemode();
    }

    return gamemode;
}
