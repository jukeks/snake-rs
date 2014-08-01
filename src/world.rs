
use snake::*;
use point::*;
use direction::*;

use std::rand;
use std::rand::Rng;

type State = Vec<Vec<Square>>;

pub struct World {
	height: uint,
	width: uint,

	state: Box<State>,
	pub food: Point,

	pub snake: Snake,

	pub ended: bool
}

pub enum Square {
	SnakeHead,
	SnakeBody,
	Food,
	Empty
}

impl World {
	fn create_state(height: uint, width: uint) -> Box<State> {
		let mut state: Box<State> = box Vec::with_capacity(width);

		for i in range(0, width) {
			let mut tmp: Vec<Square> = Vec::with_capacity(height);
			for j in range(0, height) {
				tmp.push(Empty);
			}

			state.push(tmp);
		}

		state
	}

	pub fn new(height: uint, width: uint) -> World {
		let snake = Snake::new(width/2, height/2);

		let mut w = World {height: height, width: width, state: 
			World::create_state(height, width), snake: snake, 
			food: Point {x: 0, y: 0}, ended: false};
		w.add_food();
		w.update_state();

		w
	}

	pub fn to_string(&self) -> String {
		let mut text = "".to_string();
		for i in range(0, (self.width + 1) * 2) {
			text.push_str("-");
		}
		text.push_str("\n");

		for j in range (0, self.height) {
			text.push_str("|");
			for i in range(0, self.width) {
				match (*self.state)[i][j] {
					SnakeHead	=> text.push_str("@"),
					SnakeBody 	=> text.push_str("+"),
					Food	=> text.push_str("x"),
					Empty	=> text.push_str(" ")
				}

				text.push_str(" ");
			}

			text.push_str("|\n");
		}

		for i in range(0, (self.width + 1) * 2) {
			text.push_str("-");
		}

		text.push_str(("\nPisteesi: ".to_string() + self.score().to_string()).as_slice());
		text
	}

	fn check_dead(&self) -> bool {
		let body = self.snake.body();
		// checking if snake outside of field
		for p in body.iter() {
			if p.x == 0u - 1 || p.x == self.width ||
				p.y == 0u - 1 || p.y == self.height {
				return true;
			}
		}

		// checking if snake's body parts are overlapping
		for i in range(0, body.len()) {
			let p = body[i];
			for j in range(i + 1, body.len()) {
				if body[j] == p {
					return true;
				}
			}
		}

		return false;
	}

	fn check_eating(&mut self) {
		if self.snake.head == self.food {
			self.snake.eat();
			self.add_food();
		}
	}

	fn add_food(&mut self) {
   		let mut rng = rand::task_rng();

   		// looping if collision happens
   		loop {
   			let x: uint = rng.gen_range(0, self.width);
   			let y: uint = rng.gen_range(0, self.height);
   			
   			let f = Point {x: x, y: y};
   			let mut collision = false;
   			for p in self.snake.body().iter() {
   				if *p == f {
   					collision = true;
   					break;
   				}
   			}

   			if collision {
   				continue;
   			}

   			self.food = f;
   			break;
   		}
	}

	pub fn score(&self) -> uint {
		(self.snake.len - 1) * 10
	}

	pub fn update(&mut self, direction: Direction) {
		self.snake.move(direction);
		if self.check_dead() {
			self.ended = true;
			return;
		}

		self.check_eating();
		self.state = World::create_state(self.height, self.width);
		self.update_state();
	}

	fn update_state(&mut self) {
		for p in self.snake.body().iter() {
			*self.state.get_mut(p.x).get_mut(p.y) = SnakeBody;
		}

		*self.state.get_mut(self.snake.head.x).get_mut(self.snake.head.y) = SnakeHead;
		*self.state.get_mut(self.food.x).get_mut(self.food.y) = Food;
	}
}
