
use snake::*;
use point::*;
use direction::*;

pub struct World {
	height: uint,
	width: uint,

	state: Vec<Vec<WorldState>>,
	food: Vec<Point>,

	snake: Snake,
	pub direction: Direction
}

pub enum WorldState {
	Snake,
	Food,
	Empty
}

impl World {
	fn create_state(height: uint, width: uint) -> Box<Vec<Vec<WorldState>>> {
		let mut state: Box<Vec<Vec<WorldState>>> = box Vec::with_capacity(width);

		for i in range(0, width) {
			let mut tmp: Vec<WorldState> = Vec::with_capacity(height);
			for j in range(0, height) {
				tmp.push(Empty);
			}

			state.push(tmp);
		}

		state
	}

	pub fn new(height: uint, width: uint, snake: Snake) -> World {
		World {height: height, width: width, state: *World::create_state(width, height), 
			snake: snake, direction: Down, food: vec!{}}
	}

	pub fn as_text(&self) -> String {
		let mut text = "".to_string();
		for j in range (0, self.height) {
			for i in range(0, self.width) {
				match self.state[i][j] {
					Snake 	=> text.push_str("+"),
					Food	=> text.push_str("x"),
					Empty	=> text.push_str(".")
				}

				text.push_str(" ");
			}

			text.push_str("\n");
		}

		text
	}

	fn snake_ate(&mut self) {
		let head = self.snake.head;
		let mut i = 0;
		let mut found = false;
		for f in self.food.iter() {
			if *f == head {
				found = true;
				break;
			}

			i += 1;
		}

		if found {
			self.snake.eat();
			self.food.remove(i);
		}
	}

	pub fn update(&mut self) {
		self.snake.move(self.direction, self.height, self.width);
		self.snake_ate();
		self.state = *World::create_state(self.height, self.width);
		for p in self.snake.body.iter() {
			*self.state.get_mut(p.x).get_mut(p.y) = Snake;
		}
	}
}
