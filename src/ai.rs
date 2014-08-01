
use world::*;
use direction::*;

use ncurses::*;

pub fn get_input(world: &World) -> i32 {
	let ref snake = world.snake;
	
	let dx: int = world.food.x as int - snake.head.x as int;
	let dy: int = world.food.y as int - snake.head.y as int;

	// match sections are to prevent trying to reverse, which snake can not do
	if dx < 0 {
		match snake.current_direction {
			Right => return KEY_UP,
			_	  => return KEY_LEFT,
		}
	} 
	if dx > 0 {
		match snake.current_direction {
			Left => return KEY_UP,
			_	 =>	return KEY_RIGHT,
		}
	}
	if dy < 0 {
		match snake.current_direction {
			Down => return KEY_LEFT,
			_	 =>	return KEY_UP,
		}		
	}
	if dy > 0 {
		match snake.current_direction {
			Up 	=> return KEY_RIGHT,
			_	=>	return KEY_DOWN,
		}
	}

	return -1;
}