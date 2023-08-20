use std::path::Path;

pub struct Cellar;

impl Cellar {
	pub fn all_installed() -> Vec<String> {
		std::fs::read_dir(Path::new("/opt/homebrew/Cellar"))
			.unwrap()
			.map(|e| e
				.unwrap()
				.path()
				.file_name()
				.unwrap()
				.to_str()
				.unwrap()
				.to_string())
			.collect()
	}
}