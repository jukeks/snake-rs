
use point::*;
use direction::*;

pub struct Snake {
	pub head: Point,
	body: Vec<Point>,
	pub len: uint,
	pub current_direction: Direction,
	tail: Point,
}


impl Snake {
	pub fn new(x: uint, y: uint) -> Snake {
		let p = Point {x: x, y: y};
		Snake {head: p, body: vec!{p}, 
		len: 1, tail: p, current_direction: Up}
	}

	pub fn eat(&mut self) {
		self.len += 1;
		self.body.insert(0, self.tail);
	}

	pub fn body(&self) -> &Vec<Point> {
		&self.body
	}

	pub fn move(&mut self, mut direction: Direction) {
		// disabling reversing
		match (self.current_direction, direction) {
			(Up, Down) => direction = self.current_direction,
			(Down, Up) => direction = self.current_direction,
			(Left, Right) => direction = self.current_direction,
			(Right, Left) => direction = self.current_direction,
			_ => {}
		}

		// moving
		match direction {
			Up 		=> self.head.y -= 1,
			Down	=> self.head.y += 1,
			Right	=> self.head.x += 1,
			Left 	=> self.head.x -= 1
		}

		// removing from tail and adding to head
		self.body.push(self.head);
		self.tail = self.body[0];
		self.body.remove(0);

		self.current_direction = direction;
	}
}
