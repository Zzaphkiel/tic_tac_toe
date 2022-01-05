mod game;
use game::{Board, Player};
use rand::Rng;
use std::{io, process::Command};

fn read_position_and_check(board: &Board) -> (usize, usize) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        let mut nums = input.split_whitespace();
        let mut input_position = (
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
        );

        input_position = (input_position.0 - 1, input_position.1 - 1);

        if input_position.0 > 2 || input_position.1 > 2 {
            println!("illgeal position");
            continue;
        }

        if board.filled(input_position) {
            println!("illegal position");
            continue;
        }

        return input_position;
    }
}

fn main() {
    let mut board = Board::new();
    let user_first = rand::thread_rng().gen_range(0..2) == 1;
    let mut player = match user_first {
        true => Player::Min,
        false => Player::Max,
    };
    let mut first_turn = true;

    loop {
        let _ = Command::new("cmd.exe").arg("/c").arg("cls").status();
        if (first_turn && !user_first) || !first_turn {
            let position = board.minimax_decision();
            // println!("I put at ({}, {})", position.0 + 1, position.1 + 1);

            board = board.put(player.to_chess(), position);

            board.print();
            player = player.exchange();

            if let Some(player) = board.terminal_test() {
                match player {
                    Player::Max => println!("You lose."),
                    Player::Min => println!("You Win"),
                }

                break;
            } else if board.draw_test() {
                println!("Draw");
                break;
            }
        } else {
            first_turn = false;
            board.print();
            println!("You go first");
        }

        let input_position = read_position_and_check(&board);
        board = board.put(player.to_chess(), input_position);
        player = player.exchange();

        if let Some(player) = board.terminal_test() {
            match player {
                Player::Max => println!("You lose."),
                Player::Min => println!("You Win"),
            }

            break;
        } else if board.draw_test() {
            println!("Draw");
            break;
        }
    }

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
