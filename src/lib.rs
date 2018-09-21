use std::cmp;
use std::fmt;

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
		let game = Connect4Game::from_string(
			"b
bbrrb
rrbbrr
bbrrbb
rrbbrr
bbrrbb
rrbbrr
bbrrbb"
		).unwrap();

		let tied_game = ActionResult::Tie(Connect4Game::from_string(
			"r
bbrrbb
rrbbrr
bbrrbb
rrbbrr
bbrrbb
rrbbrr
bbrrbb"
		).unwrap());
		assert_eq!( game.play_piece(0).unwrap(), tied_game)
	}
	#[test]
	fn can_win_with_4_in_a_row_across() {
		let game = Connect4Game::from_string(
			"r
r
r

r
b
b
b"
		).unwrap();

		let won_game = ActionResult::Win(Connect4Game::from_string(
			"b
r
r
r
r
b
b
b"
		).unwrap());

		assert_eq!(game.play_piece(2).unwrap(), won_game);
	}
		#[test]
	fn can_win_with_4_in_a_row_down() {
		let game = Connect4Game::from_string(
			"r
rrr
b
b
b"
		).unwrap();

		let won_game = ActionResult::Win(Connect4Game::from_string(
			"b
rrrr
b
b
b"
		).unwrap());

		assert_eq!(game.play_piece(0).unwrap(), won_game);
	}
		#[test]
	fn can_win_with_4_in_a_row_left_up() {
		let game = Connect4Game::from_string(
			"r
bbbr
bb
br
r"
		).unwrap();

		let won_game = ActionResult::Win(Connect4Game::from_string(
			"b
bbbr
bbr
br
r"
		).unwrap());

		assert_eq!(game.play_piece(1).unwrap(), won_game);
	}
		#[test]
	fn can_win_with_4_in_a_row_left_down() {
				let game = Connect4Game::from_string(
			"r
r
b
bbr
bbbr"
		).unwrap();

		let won_game = ActionResult::Win(Connect4Game::from_string(
			"b
r
br
bbr
bbbr"
		).unwrap());

		assert_eq!(game.play_piece(1).unwrap(), won_game);
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
	Red,
	Blue,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if *self == Player::Red {
			write!(f, "🔴")
		}
		else{
			write!(f, "🔵")
		}
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
	Empty,
	Piece(Player),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Cell::Piece(player) => write!(f, "{}", player),
			Cell::Empty => write!(f, "⚫"),
		}
    }
}

#[derive(Debug)]
pub enum ActionError {
	ColumnFull,
	PositionOutOfBounds,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActionResult {
	Win(Connect4Game),
	Tie(Connect4Game),
	Move(Connect4Game),
}

#[derive(Debug)]
pub enum Connect4ParseError {
	MissingPlayer,
	InvalidPlayerCharacter,
	InvalidPieceCharacter,
	RowTooLong,
	TooManyRows,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Connect4Game {
	current_player: Player,
	board: [[Cell; 6]; 7],
}
impl fmt::Display for Connect4Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}\nCurrent player:{}", self.board_to_string(), self.current_player)

    }
}
impl Connect4Game {
	pub fn new_game() -> Connect4Game {
		Connect4Game {
			current_player: Player::Red,
			board: [[Cell::Empty; 6]; 7],
		}
	}

	/*
	Format for strings:
	Fist line:
		current player
	Lines 2 - 9
	Pices, up to 6 caracters
	r = red
	b = blue

	left is bottom of board

	Example:
	r
	rbrbrb
	rbrbrb
	rbrbrb
	rbrbrb
	rbrbrb
	rbrbrb
	rbrbrb
	*/
	pub fn from_string(string: &str) -> Result<Connect4Game, Connect4ParseError> {
		let string_rows: Vec<&str> = string.split('\n').collect();
		if string_rows.len() < 1 {
			return Err(Connect4ParseError::MissingPlayer);
		}
		if string_rows.len() > 8 { // 1 player, 6 rows
			return Err(Connect4ParseError::TooManyRows);
		};

		let current_player = match string_rows[0].as_ref() {
			"r" => Player::Red,
			"b" => Player::Blue,
			_ => return {
				Err(Connect4ParseError::InvalidPlayerCharacter)
				},
		};
		let mut board = [[Cell::Empty; 6]; 7];
		for (row_index, row) in string_rows.iter().skip(1).enumerate() {
			if row.len() > 6 { // 6 peices max per row
				return Err(Connect4ParseError::RowTooLong);
			};
			for (index, caracter) in row.chars().enumerate() {
				board[row_index][index] = match caracter {
					'r' => Cell::Piece(Player::Red),
					'b' => Cell::Piece(Player::Blue),
					_ => return Err(Connect4ParseError::InvalidPieceCharacter),
				}
			}
		};
		Ok(Connect4Game {
			current_player,
			board,
		})
	}
	pub fn board_to_string(&self) -> String {
		let mut board_string: String = "".to_owned();
		for y in (0..self.board[0].len()).rev() {
			for x in 0..self.board.len() {
				board_string = format!("{}{}", board_string, self.board[x][y]);
			}
			if y > 0 {
				board_string = format!("{}\n", board_string);
			}
		}
		board_string
	}
	pub fn get_current_player(&self) -> Player {
		self.current_player
	}
	pub fn get_board(&self) -> [[Cell; 6]; 7] {
		self.board
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
		let num_up_left_moves = cmp::min(column, self.board[column].len() - 1 - current_height);
		if num_up_left_moves > 0 {
			for offset in 1..num_up_left_moves + 1 {
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
		let num_down_right_moves = cmp::min(self.board.len() - column - 1, current_height);
		if num_down_right_moves > 0 {
			for offset in 1..num_down_right_moves + 1 {
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
		// Up+right + down+left
		total = 1; // Reset total
		// Up + left
		let num_up_right_moves = cmp::min(self.board.len() - 1 - column, self.board[column].len() - 1- current_height);
		if num_up_right_moves > 0 {
			for offset in 1..num_up_right_moves + 1 {
				if let Cell::Piece(i) = self.board[column + offset][current_height + offset] {
					if i == player {
						total += 1;
					}
					else {break;}
				}
				else {break;}
			}
		}
		// Down + Left
		let num_down_left_moves = cmp::min(column, current_height);
		if num_down_left_moves > 0 {
			for offset in 1..num_down_left_moves + 1 {
				if let Cell::Piece(i) = self.board[column - offset][current_height - offset] {
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
	pub fn play_piece(&self, column: usize) -> Result<ActionResult, ActionError> {
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
						return Ok(ActionResult::Tie(new_game))
					}
					// Is board full = tie
					else if new_game.did_win_at_position(self.current_player, column, first_empty_cell) {
						return Ok(ActionResult::Win(new_game))
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
