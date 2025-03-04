use std::{
	env,
	fs::File,
	io::BufReader,
	path::{Path, PathBuf},
};

pub struct Config {}

impl Config {
	fn config_file() -> PathBuf {
		let home_folder = env::var("HOME").expect("/");
		Path::new(&home_folder).join(".ironrc")
	}

	/// Loads and evaluates `~/.ironrc` before requesting for user input
	pub fn load_file() -> Option<BufReader<File>> {
		let file_path = Self::config_file();
		File::open_buffered(file_path).ok()
	}
}
