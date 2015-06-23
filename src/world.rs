
use snake::*;
use point::*;
use direction::*;

extern crate rand;
use self::rand::Rng;

type State = Vec<Vec<Square>>;

pub struct World {
	height: usize,
	width: usize,

	state: Box<State>,
	food: Point,

	snake: Snake,

	pub ended: bool
}

pub enum Square {
	SnakeHead,
	SnakeBody,
	Food,
	Empty
}

impl World {
	fn create_state(height: usize, width: usize) -> Box<State> {
		let mut state: Box<State> = Box::new(Vec::with_capacity(width));

		for _ in (0..width) {
			let mut column: Vec<Square> = Vec::with_capacity(height);
			for _ in (0..height) {
				column.push(Square::Empty);
			}

			state.push(column);
		}

		state
	}

	pub fn new(height: usize, width: usize) -> World {
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
		for _ in (0..(self.width + 1) * 2) {
			text.push_str("-");
		}
		text.push_str("\n");

		for j in (0..self.height) {
			text.push_str("|");
			for i in (0..self.width) {
				match (*self.state)[i][j] {
					Square::SnakeHead	=> text.push_str("@"),
					Square::SnakeBody 	=> text.push_str("+"),
					Square::Food	=> text.push_str("x"),
					Square::Empty	=> text.push_str(" ")
				}

				text.push_str(" ");
			}

			text.push_str("|\n");
		}

		for _ in (0..(self.width + 1) * 2) {
			text.push_str("-");
		}

		text.push_str(&("\nPisteesi: ".to_string() + &self.score().to_string()));
		text
	}

	fn check_dead(&self) -> bool {
		let body = self.snake.body();
		// checking if snake outside of field
		for p in body.iter() {
			if p.x < 0 || p.x > ((self.width - 1) as i32) ||
				p.y < 0 || p.y > ((self.height - 1) as i32) {
				return true;
			}
		}

		// checking if snake's body parts are overlapping
		for i in (0..body.len()) {
			let p = body[i];
			for j in (i + 1..body.len()) {
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
		let mut rng = rand::thread_rng();
   		// looping if collision happens
   		loop {
   			let x: usize = rng.gen_range(0, self.width);
   			let y: usize = rng.gen_range(0, self.height);
   			
   			let f = Point {x: (x as i32), y: (y as i32)};
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

	pub fn score(&self) -> usize {
		(self.snake.len - 1) * 10
	}

	pub fn update(&mut self, direction: Direction) {
		self.snake.advance(direction);
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
			self.state[p.x as usize][p.y as usize] = Square::SnakeBody;
		}

		self.state[self.snake.head.x as usize][self.snake.head.y as usize] = Square::SnakeHead;
		self.state[self.food.x as usize][self.food.y as usize] = Square::Food;
	}
}
