
struct Point {
	x: uint,
	y: uint
}

struct Snake {
	head: Point,
	body: Vec<Point>
}

impl Snake {
	fn new(x: uint, y: uint) -> Snake {
		let p = Point {x: x, y: y};
		Snake {head: p, body: vec!{p}}
	}
}
