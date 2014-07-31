
use point::*;
use direction::*;

pub struct Snake {
	pub head: Point,
	pub body: Vec<Point>,
	history: Vec<Direction>,
	removed_point: Point,
	removed_direction: Direction
}

impl Snake {
	pub fn new(x: uint, y: uint) -> Snake {
		let p = Point {x: x, y: y};
		Snake {head: p, body: vec!{p}, history: vec!{}, 
		removed_point: p, removed_direction: Up}
	}

	pub fn eat(&mut self) {
		self.history.unshift(self.removed_direction);
		self.body.unshift(self.removed_point);
	}

	pub fn move(&mut self, mut direction: Direction, width: uint, height: uint) {
		let last: Direction = match self.history.last() {
			Some(d) 	=> *d,
			None		=> Up,
		};

		match (last, direction) {
			(Up, Down) => direction = last,
			(Down, Up) => direction = last,
			(Left, Right) => direction = last,
			(Right, Left) => direction = last,
			_ => {}
		}


		match direction {
			Up 		=> self.head.y = 
						if self.head.y == 0 { 
							0 
						} else {
							self.head.y - 1
						},
			Down	=> self.head.y = 
						if self.head.y == height - 1 {
							height - 1
						} else {
							self.head.y + 1
						},
			Right	=> self.head.x = 
						if self.head.x == width - 1 {
							width - 1
						} else {
							self.head.x + 1
						},
			Left 	=> self.head.x = 
						if self.head.x == 0 {
							0
						} else {
							self.head.x - 1
						}
		}

		self.history.push(direction);
		self.body = vec!{self.head};
	}
}
