extern crate time;

pub fn time_in_ms() -> u64 {
	time::precise_time_ns() / 1000 / 1000
}
