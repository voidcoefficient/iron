#![feature(exit_status_error)]
use std::collections::HashMap;
use std::iter::Peekable;
use std::process::{Command, exit};

use std::env;
use std::io::{self, ErrorKind, Write, stdin};
use std::path::Path;
use std::str::SplitWhitespace;

type Aliases = HashMap<String, String>;

#[derive(Clone)]
struct Shell<'a> {
	prompt: &'a str,
	aliases: Aliases,
}

impl<'a> Shell<'a> {
	fn new() -> Self {
		let aliases: Aliases = HashMap::new();
		let prompt = option_env!("IRON_PROMPT").unwrap_or("% ");
		dbg!(&aliases);

		Self { aliases, prompt }
	}

	#[allow(unconditional_recursion)]
	fn run(&'a self) {
		print!("{}", &self.prompt);
		io::stdout().flush().unwrap();

		let aliases = self.handle_input();
		let meta = Self {
			prompt: self.prompt,
			aliases,
		};

		meta.run();
	}

	fn handle_input(&'a self) -> Aliases {
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		let mut parts = input.split_whitespace().peekable();
		let command = parts.next();

		match command {
			Some("cd") => {
				let new_dir = parts.next().unwrap_or(&"/");
				let root = Path::new(new_dir);
				if let Err(e) = env::set_current_dir(root) {
					handle_generic_error(e);
				}

				self.aliases.clone()
			}
			Some("exit") => exit(0),
			Some("alias") => {
				if let None = parts.peek() {
					self
						.aliases
						.iter()
						.for_each(|(k, v)| println!("{}\t\t{}", k, v));

					return self.aliases.clone();
				}

				let mut aliases = self.aliases.clone();
				if let Some(key) = parts.next() {
					if let Some(value) = parts.next() {
						aliases.insert(key.to_owned(), value.to_owned());

						return aliases;
					}
				}

				eprintln!("usage: `alias [key] [value]`\n");
				eprintln!("example:");
				eprintln!("\t`alias ls eza` -- `ls` will now run `eza`");
				aliases
			}
			Some(command_or_alias) => {
				match self.aliases.get(command_or_alias) {
					Some(alias) => self.handle_command(alias, parts),
					None => self.handle_command(command_or_alias, parts),
				};

				self.aliases.clone()
			}
			None => {
				print!("\n");
				exit(0);
			}
		}
	}

	fn handle_command(&'a self, command: &str, parts: Peekable<SplitWhitespace<'_>>) {
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

fn handle_generic_error<E: std::error::Error>(e: E) {
	eprintln!("{}", e);
}

fn main() -> () {
	Shell::new().run();
}
