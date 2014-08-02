
use world::*;
use direction::*;
use point::*;
use snake::*;

use ncurses::*;

pub fn get_input(world: &World) -> i32 {
	let ref snake = world.snake;
	let ref food = world.food;

	let (dx_sign, dy_sign) = signs(snake, food);

	let mut decision = decide_based_on_food_direction(dx_sign, dy_sign, snake.current_direction);

	let ref snake = world.snake;
	decision = escape_detection(snake, world, decision, 0);

	decision
}

fn escape_detection(snake: &Snake, world: &World, mut decision: i32, iteration: uint) -> i32 {
	if iteration > snake.len / 2 {
		return decision;
	}

	let mut alternatives = get_alternatives(snake, decision);
	while !alternatives.is_empty() {
		decision = alternatives.pop().unwrap();
		let ref mut s = snake.clone();
		s.move(Direction::from_key(decision).unwrap());

		if about_to_collide_self(s) 
			|| about_to_collide_wall(world, s) {
			continue;
		}
		
		
		let escapable =  escape_detection(s, world, decision, iteration + 1);
		if escapable < 0 {
			continue;
		}

		return decision;
	}

	-1
}

fn get_alternatives(snake: &Snake, decision: i32) -> Vec<i32> {
	let mut alternatives = vec!{KEY_UP, KEY_DOWN, KEY_RIGHT, KEY_LEFT};
	let mut to_be_removed = vec!{decision};

	match snake.current_direction {
		Up 		=> { alternatives.push(KEY_DOWN); },
		Down 	=> { alternatives.push(KEY_UP); },
		Right 	=> { alternatives.push(KEY_LEFT); },
		Left 	=> { alternatives.push(KEY_RIGHT); },
	}
	
	while !to_be_removed.is_empty() {
		let key = to_be_removed.pop().unwrap();
		for i in range(0, alternatives.len()) {
			if alternatives[i] == key {
				alternatives.remove(i);
				break;
			}
		}
	}

	alternatives.push(decision);
	alternatives
}

fn decide_based_on_food_direction(dx_sign: int, dy_sign: int, current_direction: Direction) -> i32 {
	// the ones with direction specified are to prevent trying
	// to reverse, which the snake can't do
	match (dx_sign, dy_sign, current_direction) {
		// need to go left, but going right
		(1, 	1, 	Right) 	=> KEY_UP,
		(1,		_,	Right)	=> KEY_DOWN,

		// need to go right, but going left
		(-1, 	1, 	Left) 	=> KEY_UP,
		(-1,	_,	Left)	=> KEY_DOWN,

		// need to go up, but going down
		(1,		1, 	Down) 	=> KEY_LEFT,
		(_,		1,	Down)	=> KEY_RIGHT,

		// need to go down, but going up
		(1,		-1,	Up) 	=> KEY_LEFT,
		(_,		-1,	Up)		=> KEY_RIGHT,

		(1, 	_, 	_)		=> KEY_LEFT,
		(-1, 	_, 	_)		=> KEY_RIGHT,
		(_,		1, 	_)		=> KEY_UP,
		(_, 	-1, _)		=> KEY_DOWN,

		// no selection
		(_, 	_,	_,)		=> -1
	}
}

fn signs(snake: &Snake, food: &Point) -> (int, int) {
	let dx: int = snake.head.x as int - food.x as int;
	let dy: int = snake.head.y as int - food.y as int;

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

	(dx_sign, dy_sign)
}

fn about_to_collide_self(snake: &Snake) -> bool {
	Point::overlapping(snake.body())
}

fn about_to_collide_wall(world: &World, snake: &Snake) -> bool {
	let ref head = snake.head;
	head.x > world.width - 1 || head.y > world.height - 1
}
