use std::io::{self};

const EMPTY: char = ' ';
const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

fn print_board(board: &[char; 9]) {
    println!(" {} | {} | {} ", board[0], board[1], board[2]);
    println!("---|---|---");
    println!(" {} | {} | {} ", board[3], board[4], board[5]);
    println!("---|---|---");
    println!(" {} | {} | {} ", board[6], board[7], board[8]);
}

fn check_winner(board: &[char; 9], player: char) -> bool {
    let win_patterns = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for pattern in win_patterns.iter() {
        if board[pattern[0]] == player && board[pattern[1]] == player && board[pattern[2]] == player {
            return true;
        }
    }
    false
}

fn board_full(board: &[char; 9]) -> bool {
    !board.contains(&EMPTY)
}

fn main() {
    let mut board = [EMPTY; 9];
    let mut current_player = PLAYER_X;

    loop {
        print_board(&board);
        println!("{}'s turn. Enter position (1-9):", current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let pos: usize = match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= 9 => num - 1,
            _ => {
                println!("Invalid input! Please enter a number between 1 and 9.");
                continue;
            }
        };

        if board[pos] != EMPTY {
            println!("Invalid position!");
            continue;
        }

        board[pos] = current_player;

        if check_winner(&board, current_player) {
            print_board(&board);
            println!("{} wins!", current_player);
            break;
        }

        if board_full(&board) {
            print_board(&board);
            println!("Draw!");
            break;
        }

        current_player = if current_player == PLAYER_X { PLAYER_O } else { PLAYER_X };
    }
}
