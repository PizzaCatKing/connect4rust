extern crate connect4lib;
extern crate rand;

use std::io;
use rand::prelude::*;

use connect4lib::Connect4Game;
use connect4lib::ActionResult;
use connect4lib::Player;

pub fn main() {
	let mut game = Connect4Game::new_game();
	let mut action_result;
	let mut rng = thread_rng();
	 loop {
        println!("{}", game.board_to_string());
		let mut board_numbers: String = "".to_owned();
        for x in 0..game.get_board().len(){
			board_numbers = format!("{}{} ", board_numbers, x + 1);
		}
        println!("{}", board_numbers);

		if game.get_current_player() ==  Player::Red {
			println!("{}, Your move: ", game.get_current_player());

			let mut current_move = String::new();
			io::stdin().read_line(&mut current_move)
				.expect("Failed to read line");

			action_result = match current_move.trim().parse::<usize>() {
				Ok(current_move) => {
					if current_move < 1 {
						continue;
					}
					else{
						match game.play_piece(current_move - 1) {
							Ok(action_result) => action_result,
							Err(_) => continue,
						}
					}
				},
				Err(_) => continue,
			};
		}
		else{
			action_result = game.play_piece(rng.gen_range(0, 6)).unwrap();
		}

        match action_result {
			ActionResult::Win(new_game) => {
				println!("{}", new_game.board_to_string());
                println!("{} wins!", game.get_current_player());
                break;
            },
			ActionResult::Tie(new_game) => {
				println!("{}", new_game.board_to_string());
                println!("It's a tie, everyone loses!");
                break;
            },
			ActionResult::Move(new_game) => {
                game = new_game;
            },
		};
    }
}