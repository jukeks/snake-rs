
use point::*;
use direction::*;

pub struct Snake {
	pub head: Point,
	body: Vec<Point>,
	pub len: uint,
	last_direction: Direction,
	tail: Point,
}


impl Snake {
	pub fn new(x: uint, y: uint) -> Snake {
		let p = Point {x: x, y: y};
		Snake {head: p, body: vec!{p}, 
		len: 1, tail: p, last_direction: Up}
	}

	pub fn eat(&mut self) {
		self.len += 1;
		self.body.insert(0, self.tail);
	}

	pub fn body(&self) -> &Vec<Point> {
		&self.body
	}

	pub fn check_dead(&self, height: uint, width: uint) -> bool {
		// checking if snake outside of field
		for p in self.body.iter() {
			if p.x < 0 || p.x > width - 1 ||
				p.y < 0 || p.y > height - 1 {
				return true;
			}
		}

		// checking if snake's body parts overlapping
		for i in range(0, self.body.len()) {
			let p = self.body[i];
			for j in range(i + 1, self.body.len()) {
				if self.body[j] == p {
					return true;
				}
			}
		}

		return false;
	}

	pub fn move(&mut self, mut direction: Direction) {
		// disabling reversing
		match (self.last_direction, direction) {
			(Up, Down) => direction = self.last_direction,
			(Down, Up) => direction = self.last_direction,
			(Left, Right) => direction = self.last_direction,
			(Right, Left) => direction = self.last_direction,
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

		self.last_direction = direction;
	}
}
