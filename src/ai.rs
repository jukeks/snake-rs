
use world::*;
use direction::*;

use ncurses::*;

pub fn get_input(world: &World) -> i32 {
	let ref snake = world.snake;
	
	let dx: int = snake.head.x as int - world.food.x as int;
	let dy: int = snake.head.y as int - world.food.y as int;

	let dx_sign: int = if dx < 0 {
		-1
	} else if dx == 0 {
		0
	} else {
		1
	};
	let dy_sign: int = if dy < 0 {
		-1
	} else if dy == 0 {
		0
	} else {
		1
	};

	// the ones with direction specified are to prevent trying
	// to reverse, which the snake can't do
	match (dx_sign, dy_sign, snake.current_direction) {
		// need to go left
		(1, 	_, 	Right) 	=> KEY_UP,
		(1, 	_, 	_)		=> KEY_LEFT,

		// need to go right
		(-1, 	_, 	Left) 	=> KEY_UP,
		(-1, 	_, 	_)		=> KEY_RIGHT,

		// need to go up
		(_,		1, 	Down) 	=> KEY_LEFT,
		(_,		1, 	_)		=> KEY_UP,

		// need to go down
		(_,		-1,	Up) 	=> KEY_RIGHT,
		(_, 	-1, _)		=> KEY_DOWN,

		// no selection
		(_, 	_,	_,)		=> -1
	}
}