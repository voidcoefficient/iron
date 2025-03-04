#![feature(exit_status_error)]
use std::process::{Command, exit};

use std::env;
use std::io::{self, ErrorKind, Write};
use std::path::Path;
use std::str::SplitWhitespace;

fn handle_generic_error<E: std::error::Error>(e: E) {
	eprintln!("{}", e);
	exit(1);
}

fn handle_command(command: &str, parts: SplitWhitespace<'_>) {
	match command {
		"cd" => {
			let new_dir = parts.peekable().peek().map_or("/", |x| *x);
			let root = Path::new(new_dir);
			if let Err(e) = env::set_current_dir(root) {
				handle_generic_error(e);
			}
		}
		"exit" => exit(0),
		command => {
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
							exit(127);
						}
						_ => eprintln!("{}", e),
					};
				}
			};
		}
	}
}

fn main() -> ! {
	let prompt = option_env!("IRON_PROMPT").unwrap_or("% ");
	loop {
		print!("{}", prompt);
		io::stdout().flush().unwrap();

		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();

		let mut parts = input.split_whitespace();
		let next_command = parts.next();
		match next_command {
			Some(command) => handle_command(command, parts),
			None => {
				eprintln!("exiting");
				exit(1);
			}
		};
	}
}
