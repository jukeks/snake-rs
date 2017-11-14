
use point::*;
use direction::*;

pub struct Snake {
	pub head: Point,
	body: Vec<Point>,
	pub len: usize,
	current_direction: Direction,
	tail: Point,
}


impl Snake {
	pub fn new(x: usize, y: usize) -> Snake {
		let p = Point {x: (x as i32), y: (y as i32)};
		Snake {head: p, body: vec!{p}, 
		len: 1, tail: p, current_direction: Direction::Up}
	}

	pub fn eat(&mut self) {
		self.len += 1;
		self.body.insert(0, self.tail);
	}

	pub fn body(&self) -> &Vec<Point> {
		&self.body
	}

	pub fn advance(&mut self, mut direction: Direction) {
		// disabling reversing
		match (self.current_direction, direction) {
			(Direction::Up, Direction::Down) |
				(Direction::Down, Direction::Up) |
				(Direction::Left, Direction::Right) |
				(Direction::Right, Direction::Left) => direction = self.current_direction,

			_ => {}
		}

		// moving
		match direction {
			Direction::Up       => self.head.y -= 1,
			Direction::Down     => self.head.y += 1,
			Direction::Right    => self.head.x += 1,
			Direction::Left     => self.head.x -= 1
		}

		// removing from tail and adding to head
		self.body.push(self.head);
		self.tail = self.body[0];
		self.body.remove(0);

		self.current_direction = direction;
	}
}
