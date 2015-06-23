pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Copy  for Direction {}
impl Clone for Direction {
	fn clone(&self) -> Direction { *self }
}
