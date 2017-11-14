pub struct Point {
	pub x: i32,
	pub y: i32
}

impl PartialEq for Point {
	fn eq(&self, rhs: &Point) -> bool {
		self.x == rhs.x && self.y == rhs.y
	}
}

impl Clone for Point {
	fn clone(&self) -> Point {
		Point {x: self.x, y: self.y}
	}

	fn clone_from(&mut self, source: &Self) {
		self.x = source.x;
		self.y = source.y;
	}
}

impl Copy for Point {}
