extern crate ncurses;
extern crate time;

use ncurses::*;
use direction::*;

mod snake;
mod world;
mod direction;
mod point;

const KEY_Q: i32 = 'q' as i32;

static UPDATE_INTERVAL: u64 = 120;

fn time_in_ms() -> u64 {
	time::precise_time_ns() / 1000 / 1000
}

fn game_over() {
	timeout(-1);
	printw(" Hävisit pelin!");
	refresh();
	getch();
}

fn main() {
	let mut w = world::World::new(20, 30);

	// Start ncurses.
	initscr();
	keypad(stdscr, true);
	noecho();
	timeout(0);

	let mut next = time_in_ms() + UPDATE_INTERVAL;

	let mut direction = Direction::Up;

	loop {
		if time_in_ms() > next {
			next += UPDATE_INTERVAL;
			w.update(direction);
			if w.ended {
				game_over();
				break;
			}
		}

		match getch() {
			KEY_UP 	=> direction = Direction::Up,
			KEY_DOWN 	=> direction = Direction::Down,
			KEY_LEFT 	=> direction = Direction::Left,
			KEY_RIGHT 	=> direction = Direction::Right,
			KEY_Q 	=> break,
			_		=> {}
		}

		clear();
		printw(&w.to_string());
		refresh();

		std::thread::sleep_ms(5);
	}

	endwin();
}
