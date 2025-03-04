#![feature(exit_status_error)]
#![feature(file_buffered)]
#![feature(let_chains)]
pub mod config;
pub mod shell;
pub mod state;

use shell::Shell;
use state::State;

fn main() -> () {
	let shell = Shell::default();
	let initial_state = shell.evaluate_config(State::new());
	shell.run(initial_state);
}
