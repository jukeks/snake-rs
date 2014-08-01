
use world::*;
use direction::*;
use point::*;
use snake::*;

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
	let mut decision = 
	match (dx_sign, dy_sign, snake.current_direction) {
		// need to go left, but going right
		(1, 	1, 	Right) 	=> KEY_UP,
		(1,		_,	Right)	=> KEY_DOWN,

		// need to go right, but going left
		(-1, 	1, 	Left) 	=> KEY_UP,
		(-1,	_,	Left)	=> KEY_DOWN,

		// need to go up, but going down
		(1,		1, 	Down) 	=> KEY_LEFT,
		(-1,	_,	Down)	=> KEY_RIGHT,

		// need to go down, but going up
		(1,		-1,	Up) 	=> KEY_LEFT,
		(-1,	_,	Up)		=> KEY_RIGHT,

		(1, 	_, 	_)		=> KEY_LEFT,
		(-1, 	_, 	_)		=> KEY_RIGHT,
		(_,		1, 	_)		=> KEY_UP,
		(_, 	-1, _)		=> KEY_DOWN,

		// no selection
		(_, 	_,	_,)		=> -1
	};

	let mut alternatives = vec!{KEY_UP, KEY_DOWN, KEY_RIGHT, KEY_LEFT};
	let ref snake = world.snake;

	while !alternatives.is_empty() {
		let ref mut head = snake.head.clone();
		apply_decision(head, decision);

		if about_to_collide_self(snake, head) 
			|| about_to_collide_wall(world, head) {
			decision = alternatives.pop().unwrap();
			continue;
		}

		break;
	}

	decision
}

fn apply_decision(point: &mut Point, decision: i32) {
	match decision {
		KEY_UP 		=> point.y -= 1,
		KEY_DOWN	=> point.y += 1,
		KEY_RIGHT	=> point.x += 1,
		KEY_LEFT	=> point.x -= 1,
		_			=> {}
	}	
}

fn about_to_collide_self(snake: &Snake, head: &Point) -> bool {
	let body = snake.body();
	for p in body.iter() {
		if *head == *p {
			return true;
		}
	}

	false
}

fn about_to_collide_wall(world: &World, head: &Point) -> bool {
	head.x == 0u - 1 || head.x == world.width ||
		head.y == 0u - 1 || head.y == world.height
}
