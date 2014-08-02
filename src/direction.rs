
use ncurses::*;

pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn from_key(key: i32) -> Option<Direction> {
		match key {
			KEY_UP => Some(Up),
			KEY_DOWN => Some(Down),
			KEY_LEFT => Some(Left),
			KEY_RIGHT => Some(Right),
			_	=> None
		}
	}

	pub fn to_key(direction: Direction) -> i32 {
		match direction {
			Up 	=> KEY_UP,
			Down => KEY_DOWN,
			Left => KEY_LEFT,
			Right => KEY_RIGHT,
		}
	}
}