use std::fmt;
use std::process::exit;
use macroquad::prelude::*;

const BORDER_GAP: f32 = 10.0;
const BORDER_THICKNESS: f32 = 7.0;
const LINE_THICKNESS: f32 = 3.0; 

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
enum CellState {
	Empty,
	X,
	O,
}

impl fmt::Display for CellState {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			CellState::X => write!(f, "X"),
			CellState::O => write!(f, "O"),
			CellState::Empty => write!(f, "Empty"),
		}
	}
}



fn window_conf() -> Conf {
	Conf {
		window_title: "Tic Tac Toe".to_owned(),
		window_width: 600,
		window_height: 600,
		..Default::default()
	}
}


#[macroquad::main(window_conf)]
async fn main() {
	let mut game_state: [[CellState; 3]; 3] = [
		[CellState::Empty, CellState::Empty, CellState::Empty],
		[CellState::Empty, CellState::Empty, CellState::Empty],
		[CellState::Empty, CellState::Empty, CellState::Empty],
	];

	let mut current_player = CellState::X;
	let mut game_over = WinState::None;

	loop {
		if (is_key_down(KeyCode::RightSuper) || is_key_down(KeyCode::LeftSuper)) && is_key_down(KeyCode::Q) {
			exit(0);
		}

		clear_background(BLACK);
		draw_main_border(screen_width());

		if game_over.ne(&WinState::None) {
			display_game_over(&game_over);
			next_frame().await;
			continue;
		}

		let result = handle_tic_tac_toe_board(100.0, 100.0, screen_width() - 200.0, screen_height() - 200.0, &game_state);

		match result {
			// The division was valid
			Some((cell_x, cell_y)) => {
				if matches!(game_state[cell_y][cell_x], CellState::Empty) {
					game_state[cell_y][cell_x] = current_player;
					
					if matches!(current_player, CellState::X) { current_player = CellState::O }
					else { current_player = CellState::X }

					let result = check_game_win(game_state);

					game_over = result;

					// if result.ne(&WinState::None) {
						// reset_board(&mut game_state);
						// current_player = CellState::X;
					// }

					println!("{current_player}");
				}
			},
			// The division was invalid
			None => (),
		}

		if game_over.ne(&WinState::None) {
			display_game_over(&game_over)
		}

		next_frame().await;
	}
}

fn display_game_over(winner: &WinState) {
	let text_dimensions = measure_text("Game Over", None, 50, 1.0);
	draw_text("Game Over", (screen_width() - text_dimensions.width) / 2.0, (screen_height() - 15.0) / 2.0, 50.0, WHITE);

	let winner_ypos = (screen_height() + text_dimensions.height + 45.0) / 2.0;
	match winner {
		WinState::X => {
			let text_dimensions = measure_text("X Wins!", None, 50, 1.0);
			draw_text("X Wins!", (screen_width() - text_dimensions.width) / 2.0, winner_ypos, 50.0, WHITE);
		},
		WinState::O => {
			let text_dimensions = measure_text("O Wins!", None, 50, 1.0);
			draw_text("O Wins!", (screen_width() - text_dimensions.width) / 2.0, winner_ypos, 50.0, WHITE);
		},
		WinState::Tie => {
			let text_dimensions = measure_text("Tie!", None, 50, 1.0);
			draw_text("Tie!", (screen_width() - text_dimensions.width) / 2.0, winner_ypos, 50.0, WHITE);
		},
		WinState::None => (),
	}
}

fn reset_board(game_state: &mut [[CellState; 3]; 3]) {
	for x in 0..game_state.len() {
		for y in 0..game_state.len() {
			game_state[x][y] = CellState::Empty;
		}
	}
}

#[derive(PartialEq)]
enum WinState {
	X,
	O,
	Tie,
	None,
}

fn check_game_win(game_state: [[CellState; 3]; 3]) -> WinState {
	// Helper function to check if all cells in a line (row, column, or diagonal) are the same
	fn check_line(game_state: &[[CellState; 3]; 3], line: &[(usize, usize)], cell_state: CellState) -> bool {
		line.iter().all(|&(row, col)| game_state[row][col] == cell_state)
	}

	// Check rows and columns
	for i in 0..3 {
		if check_line(&game_state, &[(i, 0), (i, 1), (i, 2)], CellState::X)
			|| check_line(&game_state, &[(0, i), (1, i), (2, i)], CellState::X) {
			return WinState::X
		}
		if check_line(&game_state, &[(i, 0), (i, 1), (i, 2)], CellState::O)
			|| check_line(&game_state, &[(0, i), (1, i), (2, i)], CellState::O) {
			return WinState::O
		}
	}

	// Check diagonals
	if check_line(&game_state, &[(0, 0), (1, 1), (2, 2)], CellState::X)
		|| check_line(&game_state, &[(0, 2), (1, 1), (2, 0)], CellState::X) {
			return WinState::X;
	}
	if check_line(&game_state, &[(0, 0), (1, 1), (2, 2)], CellState::O)
		|| check_line(&game_state, &[(0, 2), (1, 1), (2, 0)], CellState::O) {
			return WinState::O
	}

	if (0..3).all(|x| (0..3).all(|y| !matches!(game_state[x][y], CellState::Empty))) {
		return WinState::Tie
	}

	WinState::None
}

fn handle_tic_tac_toe_board(x: f32, y: f32, width: f32, height: f32, game_state: &[[CellState; 3]; 3]) -> Option<(usize, usize)> {
	let cell_width = width / 3.0;
	let cell_height = height / 3.0;

	for i in 1..3 {
		let line_x = x + cell_width * i as f32;
		draw_line(line_x, y, line_x, y + height, LINE_THICKNESS, WHITE);
	}

	for i in 1..3 {
		let line_y = y + cell_height * i as f32;
		draw_line(x, line_y, x + width, line_y, LINE_THICKNESS, WHITE);
	}

	for row in 0..3 {
		for col in 0..3 {
			match game_state[row][col] {
				CellState::X => draw_x(x + col as f32 * cell_width, y + row as f32 * cell_height, cell_width, cell_height),
				CellState::O => draw_o(x + col as f32 * cell_width, y + row as f32 * cell_height, cell_width, cell_height),
				CellState::Empty => (),
			}
		}
	}

	if is_mouse_button_pressed(MouseButton::Left) {
		let (mouse_x, mouse_y) = mouse_position();

		if mouse_x < x || mouse_y < y || mouse_x > x + width || mouse_y > y + height { return None };

		let cell_x = ((mouse_x - x) / cell_width).floor() as usize;
		let cell_y = ((mouse_y - y) / cell_height).floor() as usize;

		return Some((cell_x, cell_y));
	}

	None
}


// Draws an X in the specified cell
fn draw_x(x: f32, y: f32, width: f32, height: f32) {
	let padding = 10.0; // Padding from the cell border
	draw_line(x + padding, y + padding, x + width - padding, y + height - padding, 3.0, WHITE);
	draw_line(x + width - padding, y + padding, x + padding, y + height - padding, 3.0, WHITE);
}

// Draws an O (circle) in the specified cell
fn draw_o(x: f32, y: f32, width: f32, height: f32) {
	let radius = width.min(height) / 2.0 - 10.0; // Radius of the circle
	let center_x = x + width / 2.0;
	let center_y = y + height / 2.0;

	draw_circle_lines(center_x, center_y, radius, 3.0, WHITE);
}

fn draw_main_border(width: f32) {
	draw_rectangle_lines(
		BORDER_GAP,
		BORDER_GAP,
		width - (BORDER_GAP + BORDER_THICKNESS),
		screen_height() - (BORDER_GAP + BORDER_THICKNESS),
		BORDER_THICKNESS,
		WHITE,
	);
}

