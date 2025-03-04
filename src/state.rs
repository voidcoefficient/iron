use std::{collections::HashMap, env};

pub type Aliases = HashMap<String, String>;

#[derive(Clone)]
pub struct State {
	pub prompt: String,
	pub aliases: Aliases,
}

impl State {
	pub fn new() -> Self {
		let aliases: Aliases = HashMap::new();
		let prompt = env::var("IRON_PROMPT").unwrap_or("% ".to_string());

		Self { aliases, prompt }
	}

	pub fn insert_alias(&self, key: String, value: String) -> Self {
		let mut aliases = self.aliases.clone();
		aliases.insert(key.to_owned(), value.to_owned());

		return State {
			prompt: self.prompt.clone(),
			aliases,
		};
	}
}
