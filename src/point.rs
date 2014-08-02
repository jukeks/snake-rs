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

impl Point {
	pub fn overlapping(points: &Vec<Point>) -> bool {
		for i in range(0, points.len()) {
			let p = points[i];
			for j in range(i + 1, points.len()) {
				if points[j] == p {
					return true;
				}
			}
		}

		false
	}
}
