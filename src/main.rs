use text_io::read;
use std::cmp;
use std::sync::{Mutex};
use rayon::prelude::*;
use tokio::time::Instant;

const N: usize = 4;

fn print_board(board: &[char]) {
    println!();

    for i in 1..N + 1 {
        for x in 1..N + 1 {
            print!(" {} ", board[(i - 1) * N + x]);

            if x < N {
                print!("|");
            }
        }
        println!();
        if i < N {
            println!("{}", String::from("----").repeat(N));
        }
    }
}

fn is_end(board: &Vec<char>) -> bool {
    // diagonals

    // top-left to bottom-right
    let mut sol = 0;

    for i in 1..(N + 1) {
        if board[N * (i - 1) + i] == 'X' {
            sol += 1;
        }
    }

    if sol == N {
        return true;
    }

    // top-right to bottom-left
    sol = 0;

    for i in 1..(N + 1) {
        if board[N * (i - 1) + (N - i + 1)] == 'X' {
            sol += 1;
        }
    }

    if sol == N {
        return true;
    }

    // rows
    for ze in 1..(N + 1) {
        sol = 0;

        let mut x = 1 + (ze - 1) * N;
        for _i in 1..(N + 1) {
            if board[x] == 'X' {
                sol += 1;
            }
            x += 1
        }

        if sol == N {
            return true;
        }
    }

    // columns
    for sp in 1..(N + 1) {
        sol = 0;

        let mut x = 1 + (sp - 1);
        for _i in 1..(N + 1) {
            if board[x] == 'X' {
                sol += 1
            }
            x += N;
        }

        if sol == N {
            return true;
        }
    }

    false
}

fn is_board_full(board:  &Vec<char>) -> bool {
    !board.contains(&' ')
}


fn mark_square(player: char, position: usize, board: &mut Vec<char>) {
    board[position] = player;
}

fn player_move(board: &mut Vec<char>) {
    let mut run = true;

    while run {
        print!("Bitte wähle eine Position für dein nächstes 'X' (1-{}): ", N*N);
        let move_input: String = read!("{}\n");

        if let Ok(move_input) = move_input.parse::<usize>() {
            if (0 < move_input) && move_input < (N*N+1) {
                if board[move_input] == ' ' {
                    run = false;
                    mark_square('X', move_input, board);
                } else {
                    println!("Sorry, diese Position ist schon besetzt!");
                }
            } else {
                println!("Du musst bitte eine Position von 1 bis {} wählen!", N*N)
            }
        } else {
            println!("Bitte gib eine Zahl ein!")
        }
    }
}

fn comp_move(board: &Vec<char>) -> usize {
    let possible_moves: Vec<_> = board.iter().enumerate().filter(|&(x, c)| *c == ' ' && x > 0).collect();

    println!("Mögliche Züge für die \"KI\": {:?}", possible_moves);

    let best_score = Mutex::new(i32::MIN);
    let best_move = Mutex::new(0);

    let board_copy = board.clone();

    possible_moves.into_par_iter().for_each(|cmove| {

        let mut move_board_copy = board_copy.clone();

        move_board_copy[cmove.0] = 'X';

        // use minimax
        let score = minimax_alpha_beta(&board_copy, i32::MIN, i32::MAX, 0, false);
        println!("Bewertung von Zug {}: {}", cmove.0, score);

        let mut score_res =  best_score.lock().unwrap();
        let mut best_move_res =  best_move.lock().unwrap();
        if score > *score_res {
            *score_res = score;
            *best_move_res = cmove.0;
            println!("{:?}", cmove);
        }

        move_board_copy[cmove.0] = ' ';
    });

    let return_value = best_move.lock().unwrap();

    *return_value
}

fn minimax_alpha_beta(curr_board: &Vec<char>, mut alpha: i32, mut beta: i32, depth: i32, is_maximizing: bool) -> i32 {
    // terminal states

    let mut curr_board = curr_board.to_vec();

    if is_end(&curr_board) && is_maximizing {
        return 1;
    } else if is_end(&curr_board) && !is_maximizing {
        return -1;
    }

    let possible_moves: Vec<_> = curr_board.clone().into_iter().enumerate().filter(|&(x, c)| c == ' ' && x > 0).collect();
    if is_maximizing {
        let mut best_score = i32::MIN;
        for cmove in possible_moves {
            curr_board[cmove.0] = 'X';
            let score = minimax_alpha_beta(&curr_board, alpha, beta, depth + 1, false);

            curr_board[cmove.0] = ' ';
            best_score = cmp::max(score, best_score);

            alpha = cmp::max(alpha, best_score);

            if beta <= alpha {
                break
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;
        for cmove in possible_moves {
            curr_board[cmove.0] = 'X';
            let score = minimax_alpha_beta(&curr_board, alpha, beta, depth + 1, true);

            curr_board[cmove.0] = ' ';
            best_score = cmp::min(score, best_score);

            beta = cmp::min(beta, best_score);

            if beta <= alpha {
                break
            }
        }

        best_score
    }
}

#[tokio::main]
async fn main() {
    print!("Willst du spielen? (y/n) ");
    let answer: String = read!("{}\n");

    if answer.to_lowercase() == "y" || answer.to_lowercase() == "yes" {
        let mut board = [' '; (N * N + 1)].to_vec();
        println!("---------------------------------------");
        game(&mut board);

    }
}

fn game(board: &mut Vec<char>) {


    println!("Willkommen zu {}-Verliert!", N);
    print_board(board);

    while !is_board_full(board) {
        if !is_end(board) {
            player_move(board);
            print_board(board);
        } else {
            println!("Du hast gewonnen! Gute Arbeit!");
            break
        }

        if !is_end(board) {
            // time
            let start = Instant::now();
            let cmove = comp_move(board);
            let duration = start.elapsed();

            println!("Das dauerte: {:?}", duration);

            mark_square('X', cmove, board);
            println!("Die künstliche \"Intelligenz\" setzt 'X' auf Position {}: ", cmove);
            print_board(board);
        } else {
            println!("Sorry, dieses mal hat die KI gewonnen!");
            break
        }
    }

}
