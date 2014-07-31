pub struct Point {
	pub x: uint,
	pub y: uint
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
}