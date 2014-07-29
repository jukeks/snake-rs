pub struct World {
	height: uint,
	width: uint,

	state: Vec<Vec<WorldState>>
}

pub enum WorldState {
	Snake,
	Food,
	Empty
}

impl World {
	pub fn new(height: uint, width: uint) -> World {
		let mut v: Vec<Vec<WorldState>> = Vec::with_capacity(width);

		for i in range(0, width) {
			let mut tmp: Vec<WorldState> = Vec::with_capacity(height);
			for j in range(0, height) {
				tmp.push(Empty);
			}

			v.push(tmp);
		}

		World {height: height, width: width, state: v}
	}

	pub fn as_text(&self) -> String {
		let mut text = "".to_string();
		for i in range (0, self.width) {
			for j in range(0, self.height) {
				match self.state[i][j] {
					Snake 	=> text.push_str("+"),
					Food	=> text.push_str("x"),
					Empty	=> text.push_str(" ")
				}

				text.push_str(" ");
			}

			text.push_str("\n");
		}

		text
	}
}