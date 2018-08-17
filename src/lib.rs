use std::cmp;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_game_is_empty() {
		let new_game = Connect4Game::new_game();
		assert_eq!(new_game.current_player, Player::Red);
	}

	#[test]
	fn playing_a_piece_doesnt_change_the_original_object() {
		let game = Connect4Game::new_game();
		game.play_piece(0).unwrap();

		assert_eq!(game.board[0][0], Cell::Empty);
		assert_eq!(game.current_player, Player::Red);

	}

	#[test]
	fn playing_a_piece_adds_it_to_the_column() {
		let game = Connect4Game::new_game();
		if let ActionResult::Move(game_after_move) = game.play_piece(0).unwrap() {
			assert_eq!(game_after_move.board[0][0], Cell::Piece(Player::Red));
			assert_eq!(game_after_move.current_player, Player::Blue);
		}
		else{
			panic!("Playing piece on empty board caused result other than 'Move'");
		}
	}

	#[test]
	fn playing_a_piece_causing_a_full_board_results_in_a_tie() {
		// Col 0 has 1 missing piece
		let game = Connect4Game {
			current_player: Player::Red,
			board: [
				[Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Piece(Player::Red), Cell::Piece(Player::Red), Cell::Piece(Player::Blue), Cell::Empty],
				[Cell::Piece(Player::Red), Cell::Piece(Player::Red), Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Piece(Player::Red), Cell::Piece(Player::Red)],
				[Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Piece(Player::Red), Cell::Piece(Player::Red), Cell::Piece(Player::Blue), Cell::Piece(Player::Blue)],
				[Cell::Piece(Player::Red), Cell::Piece(Player::Red), Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Piece(Player::Red), Cell::Piece(Player::Red)],
				[Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Piece(Player::Red), Cell::Piece(Player::Red), Cell::Piece(Player::Blue), Cell::Piece(Player::Blue)],
				[Cell::Piece(Player::Red), Cell::Piece(Player::Red), Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Piece(Player::Red), Cell::Piece(Player::Red)],
				[Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Piece(Player::Red), Cell::Piece(Player::Red), Cell::Piece(Player::Blue), Cell::Piece(Player::Blue)],
			],
		};
		match game.play_piece(0) {
			Ok(n)  => assert_eq!(n, ActionResult::Tie),
			Err(_) => assert!(false)
		}
	}
		#[test]
	fn can_win_with_4_in_a_row_across() {
		let game = Connect4Game {
			current_player: Player::Red,
			board: [
				[Cell::Piece(Player::Red), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Red), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Red), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Blue), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Blue), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Blue), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
			],
		};

		assert_eq!(game.play_piece(2).unwrap(), ActionResult::Win);
	}
		#[test]
	fn can_win_with_4_in_a_row_down() {
		let game = Connect4Game {
			current_player: Player::Red,
			board: [
				[Cell::Piece(Player::Red), Cell::Piece(Player::Red), Cell::Piece(Player::Red), Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Blue), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Blue), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Blue), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
			],
		};

		assert_eq!(game.play_piece(0).unwrap(), ActionResult::Win);
	}
	fn can_win_with_4_in_a_row_left_up() {
		let game = Connect4Game {
			current_player: Player::Red,
			board: [
				[Cell::Piece(Player::Red), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Blue), Cell::Piece(Player::Red), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Piece(Player::Red), Cell::Empty, Cell::Empty],
				[Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
			],
		};

		assert_eq!(game.play_piece(2).unwrap(), ActionResult::Win);
	}
	fn can_win_with_4_in_a_row_left_down() {
		let game = Connect4Game {
			current_player: Player::Red,
			board: [
				[Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Piece(Player::Red), Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Blue), Cell::Piece(Player::Blue), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Blue), Cell::Piece(Player::Red), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Piece(Player::Red), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
				[Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
			],
		};

		assert_eq!(game.play_piece(2).unwrap(), ActionResult::Win);
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Player {
	Red,
	Blue,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
	Empty,
	Piece(Player),
}

#[derive(Debug)]
enum ActionError {
	ColumnFull,
	PositionOutOfBounds,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ActionResult {
	Win,
	Tie,
	Move(Connect4Game)
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Connect4Game {
	current_player: Player,
	board: [[Cell; 6]; 7],
}

impl Connect4Game {
	fn new_game() -> Connect4Game {
		Connect4Game {
			current_player: Player::Red,
			board: [[Cell::Empty; 6]; 7],
		}
	}
	fn get_top_row_for_column(&self, column: usize) -> Option<usize> {
		self.board[column].iter().position(|&r| r == Cell::Empty)
	}
	fn did_win_at_position(&self, player: Player, column: usize, current_height: usize) -> bool {
		let total_to_win = 4;
		let mut total=1; // Current piece counts as 1
		// Left
		if column > 0 {
			for x in (0..column).rev() {
				if let Cell::Piece(i) = self.board[x][current_height] {
					if i == player {
						total += 1;
					}
					else {break;}
				}
				else {break;}
			}
		}
		//Right
		if column < self.board.len()-1 {
			for x in column+1..(self.board.len()-1) {
				if let Cell::Piece(i) = self.board[x][current_height] {
					if i == player {
						total += 1;
					}
					else { break; }
				}
				else {break;}
			}
		}

		if total >= total_to_win {
			return true
		}
		//Down
		total = 1; // Reset total

		if current_height >= total_to_win -1 {
			for x in (0..current_height).rev() {
				if let Cell::Piece(i) = self.board[column][x] {
					if i == player {
						total += 1;
					}
					else { break; }
				}
				else { break; }
			}
		}
		if total >= total_to_win {
			return true
		}
		// Up+left + down+right
		total = 1; // Reset total
		// Up + left
		let num_left_up_moves = cmp::min(column, self.board[column].len() - 1 - current_height);
		if num_left_up_moves > 0 {
			for offset in (1..num_left_up_moves) {
				if let Cell::Piece(i) = self.board[column - offset][current_height + offset] {
					if i == player {
						total += 1;
					}
					else {break;}
				}
				else {break;}
			}
		}
		// Down + Right
		let num_down_right_moves = cmp::min(self.board.len() - 1 - column, current_height);
		if num_down_right_moves > 0 {
			for offset in (1..num_down_right_moves) {
				if let Cell::Piece(i) = self.board[column + offset][current_height - offset] {
					if i == player {
						total += 1;
					}
					else {break;}
				}
				else {break;}
			}
		}

		if total >= total_to_win {
			return true
		}
		if current_height >= total_to_win {
			for x in (0..current_height).rev() {
				println!("Down {:?} col:{} height:{}, {}", self.board[x][current_height], x, current_height, total);
				if let Cell::Piece(i) = self.board[column][x] {
					if i == player {
						total += 1;
					}
					else { break; }
				}
				else { break; }
			}
		}
		if total >= total_to_win {
			return true
		}
		// Up+left + down+right
		total = 1; // Reset total
		// Up + left
		let num_left_up_moves = cmp::min(column, self.board[column].len() - 1 - current_height);
		if num_left_up_moves > 0 {
			for offset in 1..num_left_up_moves {
				if let Cell::Piece(i) = self.board[column - offset][current_height + offset] {
					if i == player {
						total += 1;
					}
					else {break;}
				}
				else {break;}
			}
		}
		// Down + Right
		let num_down_right_moves = cmp::min(self.board.len() - 1 - column, current_height);
		if num_down_right_moves > 0 {
			for offset in 1..num_down_right_moves {
				if let Cell::Piece(i) = self.board[column + offset][current_height - offset] {
					if i == player {
						total += 1;
					}
					else {break;}
				}
				else {break;}
			}
		}

		if total >= total_to_win {
			return true
		}

		false
	}
	fn is_board_full(&self) -> bool {
		// If all top rows are full the board is full
		!self.board.iter().any(|column| column[column.len() - 1] == Cell::Empty)
	}
	// Add current player's piece to the lowest position in the column
	fn play_piece(&self, column: usize) -> Result<ActionResult, ActionError> {
		if column >= self.board.len() {
			Err(ActionError::PositionOutOfBounds)
		}
		else{
			match self.get_top_row_for_column(column) {

				Some(first_empty_cell) => {
					let new_game = Connect4Game {
						current_player: {
							if self.current_player == Player::Red {
								Player::Blue
							}
							else{
								Player::Red
							}
						},
						board: {
							let mut board = self.board.clone();
							board[column][first_empty_cell] = Cell::Piece(self.current_player);
							board
						}
					};
					// Did achieve 4 in a row = win
					if new_game.is_board_full() {
						return Ok(ActionResult::Tie)
					}
					// Is board full = tie
					else if new_game.did_win_at_position(self.current_player, column, first_empty_cell) {
						return Ok(ActionResult::Win)
					}
					// Else next move
					else {
						return Ok(ActionResult::Move(new_game))
					}
				},
				None => Err(ActionError::ColumnFull)
			}
		}
	}
}