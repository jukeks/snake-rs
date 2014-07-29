
#![feature(globs)]

extern crate ncurses;

use ncurses::*;

mod snake;
mod world;

fn main() {
	let w = world::World::new(10, 10);

	/* Start ncurses. */
	initscr();

	/* Print to the back buffer. */
	printw(w.as_text().as_slice());

	/* Update the screen. */
	refresh();

	/* Wait for a key press. */
	getch();

	/* Terminate ncurses. */
	endwin();
}
