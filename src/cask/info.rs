#![allow(dead_code)]

pub struct Info;
impl Info {
	pub fn get_info<S: AsRef<str> + std::fmt::Display>(cask: S) {
		println!("Getting {cask} info");
	}
}