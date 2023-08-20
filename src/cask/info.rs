use std::fmt::Display;

pub struct Info {

}
impl Info {
	pub fn get_info<S: AsRef<str> + Display>(cask: S) {
		println!("Getting {cask} info");
	}
}