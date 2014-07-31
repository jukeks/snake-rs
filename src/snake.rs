
use point::*;
use direction::*;

pub struct Snake {
	pub head: Point,
	history: Vec<History>,
	pub len: uint,
	popped_history: History,
}

struct History {
	p: Point,
	d: Direction,
}

impl Snake {
	pub fn new(x: uint, y: uint) -> Snake {
		let p = Point {x: x, y: y};
		let history = History {p: p, d: Up};
		Snake {head: p, history: vec!{history}, 
		len: 1, popped_history: history}
	}

	pub fn eat(&mut self) {
		self.len += 1;
		self.history.insert(0, self.popped_history);
	}


	pub fn body(&self) -> Vec<Point> {
		let mut v: Vec<Point> = vec!{};
		for h in self.history.iter() {
			v.push(h.p);
		}

		v
	}

	pub fn check_dead(&self, height: uint, width: uint) -> bool {
		let mut body = self.body();
		for p in body.iter() {
			if p.x < 0 || p.x > width - 1 ||
				p.y < 0 || p.y > height - 1 {
				return true;
			}
		}

		while !body.is_empty() {
			let h = body.pop().unwrap();

			for p in body.iter() {
				if *p == h {
					return true;
				}
			}
		}

		return false;
	}

	pub fn move(&mut self, mut direction: Direction, height: uint, width: uint) {
		let last = match self.history.last() {
			Some(d) 	=> *d,
			None		=> History {p: self.head, d: Up},
		};

		// disabling reversing
		match (last.d, direction) {
			(Up, Down) => direction = last.d,
			(Down, Up) => direction = last.d,
			(Left, Right) => direction = last.d,
			(Right, Left) => direction = last.d,
			_ => {}
		}

		// moving
		match direction {
			Up 		=> self.head.y -= 1,
			Down	=> self.head.y += 1,
			Right	=> self.head.x += 1,
			Left 	=> self.head.x -= 1
		}


		self.history.push(History{p: self.head, d: direction});
		self.popped_history = self.history[0];
		self.history.remove(0);
	}
}
