
#![feature(globs)]

extern crate ncurses;
extern crate time;

use ncurses::*;

mod snake;
mod world;
mod direction;
mod point;

static KEY_Q: i32 = 'q' as i32;
static KEY_W: i32 = 'w' as i32;
static KEY_A: i32 = 'a' as i32;
static KEY_S: i32 = 's' as i32;
static KEY_D: i32 = 'd' as i32;

fn time_in_ms() -> u64 {
	time::precise_time_ns() / 1000 / 1000
}

fn main() {
	let s = snake::Snake::new(5, 1);
	let mut w = world::World::new(10, 10, s);
	w.update();

	/* Start ncurses. */
	initscr();
	noecho();

	timeout(0);
	let mut next = time_in_ms() + 200;

	loop {
		if time_in_ms() > next {
			next += 150;
			w.update();
		}

		clear();

		printw(w.as_text().as_slice());

		let c = getch();

		match c {
			KEY_W 	=> w.direction = direction::Up,
			KEY_S 	=> w.direction = direction::Down,
			KEY_A 	=> w.direction = direction::Left,
			KEY_D 	=> w.direction = direction::Right,
			KEY_Q 	=> break,
			_		=> {printw("did not get key");}
		}

		printw(c.to_string().as_slice());

		refresh();

		std::io::timer::sleep(20);
	}

	endwin();
}
