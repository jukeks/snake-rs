pub struct Point {
	pub x: uint,
	pub y: uint
}

impl PartialEq for Point {
	fn eq(&self, rhs: &Point) -> bool {
		self.x == rhs.x && self.y == rhs.y
	}
}