use std::iter::Peekable;
use std::process::{Command, exit};

use std::env;
use std::io::{self, BufRead, ErrorKind, Write, stdin};
use std::path::Path;
use std::str::SplitWhitespace;

use crate::config::Config;
use crate::state::State;

fn handle_generic_error<E: std::error::Error>(e: E) {
	eprintln!("{}", e);
}

#[derive(Default)]
pub struct Shell {}

impl Shell {
	pub fn run_once(&self, state: State) -> State {
		print!("{}", &state.prompt);
		io::stdout().flush().unwrap();

		self.handle_input(state)
	}

	pub fn evaluate_config(&self, state: State) -> State {
		if let Some(config_file) = Config::load_file() {
			return config_file.lines().fold(state, |acc, x| {
				self.evaluate(
					acc,
					x.unwrap_or("".to_string()).split_whitespace().peekable(),
				)
			});
		}

		state.clone()
	}

	#[allow(unconditional_recursion)]
	pub fn run(&self, state: State) {
		let new_state = self.run_once(state);
		self.run(new_state);
	}

	fn evaluate(&self, state: State, mut parts: Peekable<SplitWhitespace<'_>>) -> State {
		let command = parts.next();

		match command {
			Some("cd") => {
				let new_dir = parts.next().unwrap_or(&"/");
				let root = Path::new(new_dir);
				if let Err(e) = env::set_current_dir(root) {
					handle_generic_error(e);
				}

				state.clone()
			}
			Some("exit") => exit(0),
			Some("alias") => {
				if let None = parts.peek() {
					state
						.aliases
						.iter()
						.for_each(|(k, v)| println!("{}\t\t{}", k, v));

					return state.clone();
				}

				if let Some(key) = parts.next()
					&& let Some(value) = parts.next()
				{
					return state.insert_alias(key.to_owned(), value.to_owned());
				}

				eprintln!("usage: `alias [key] [value]`\n");
				eprintln!("example:");
				eprintln!("\t`alias ls eza` -- `ls` will now run `eza`");
				state.clone()
			}
			Some(command_or_alias) => {
				match state.aliases.get(command_or_alias) {
					Some(alias) => self.handle_command(alias, parts),
					None => self.handle_command(command_or_alias, parts),
				};

				state.clone()
			}
			None => {
				print!("\n");
				exit(0);
			}
		}
	}

	fn handle_input(&self, state: State) -> State {
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		let parts = input.split_whitespace().peekable();

		self.evaluate(state, parts)
	}

	fn handle_command(&self, command: &str, parts: Peekable<SplitWhitespace<'_>>) {
		let child = Command::new(command).args(parts).spawn();
		match child {
			Ok(mut child) => match child.wait() {
				Ok(exit_status) => match exit_status.exit_ok() {
					Err(e) => handle_generic_error(e),
					Ok(_) => {}
				},
				Err(e) => handle_generic_error(e),
			},
			Err(e) => {
				match e.kind() {
					ErrorKind::NotFound => {
						eprintln!("commmand not found: {}", command);
					}
					_ => eprintln!("{}", e),
				};
			}
		};
	}
}
