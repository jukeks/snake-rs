
#![feature(globs)]

extern crate ncurses;
extern crate time;

use ncurses::*;

use std::os;

mod snake;
mod world;
mod direction;
mod point;
mod ai;

static KEY_Q: i32 = 'q' as i32;

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
	let mut ai_game = false;
	let args = os::args();
	if args.len() > 1 {
		ai_game = args[1] == "-ai".to_string();
	}

	game_loop(ai_game);
}

fn game_loop(ai_game: bool) {
	let mut w = world::World::new(20, 30);

	// Start ncurses.
	initscr();
	keypad(stdscr, true);
	noecho();
	timeout(0);

	let mut next = time_in_ms() + UPDATE_INTERVAL;

	let mut direction = direction::Up;

	loop {
		if time_in_ms() > next {
			next += UPDATE_INTERVAL;
			w.update(direction);
			if w.ended {
				game_over();
				break;
			}

			clear();
			printw(w.to_string().as_slice());
			refresh();
		}

		let input = if ai_game {
			ai::get_input(&w)
		} else {
			getch()
		};

		match input {
			KEY_UP 	=> direction = direction::Up,
			KEY_DOWN 	=> direction = direction::Down,
			KEY_LEFT 	=> direction = direction::Left,
			KEY_RIGHT 	=> direction = direction::Right,
			KEY_Q 	=> break,
			_		=> {}
		}

		std::io::timer::sleep(5);
	}

	endwin();
}
