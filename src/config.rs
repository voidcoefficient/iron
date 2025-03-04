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

	pub fn load_file_from_path(file_path: PathBuf) -> Option<BufReader<File>> {
		File::open_buffered(file_path).ok()
	}

	pub fn load_file_from_path_string(file_path: String) -> Option<BufReader<File>> {
		Self::load_file_from_path(PathBuf::from(&file_path))
	}

	/// Loads and evaluates `~/.ironrc` before requesting for user input
	pub fn load_file() -> Option<BufReader<File>> {
		let file_path = Self::config_file();
		Self::load_file_from_path(file_path)
	}
}
