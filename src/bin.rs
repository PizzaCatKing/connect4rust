extern crate connect4lib;

use std::io;

use connect4lib::Connect4Game;
use connect4lib::ActionResult;

pub fn main() {
	let mut game = Connect4Game::new_game();

	 loop {
        println!("{}", game);
        println!("Your move: ");

        let mut current_move = String::new();

        io::stdin().read_line(&mut current_move)
            .expect("Failed to read line");

        let action_result = match current_move.trim().parse() {
            Ok(current_move) => {
				match game.play_piece(current_move) {
					Ok(action_result) => action_result,
					Err(_) => continue,
				}
			},
            Err(_) => continue,
        };

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