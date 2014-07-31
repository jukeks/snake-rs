
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
	noecho();
	timeout(0);

	let mut next = time_in_ms() + UPDATE_INTERVAL;

	loop {
		if time_in_ms() > next {
			next += UPDATE_INTERVAL;
			w.update();
			if w.ended {
				game_over();
				break;
			}
		}

		match getch() {
			KEY_W 	=> w.direction = direction::Up,
			KEY_S 	=> w.direction = direction::Down,
			KEY_A 	=> w.direction = direction::Left,
			KEY_D 	=> w.direction = direction::Right,
			KEY_Q 	=> break,
			_		=> {}
		}

		clear();
		printw(w.to_string().as_slice());
		refresh();

		std::io::timer::sleep(5);
	}

	endwin();
}
