#![feature(lazy_cell)]

mod cask;
mod formula;
mod cellar;

use std::error::Error;
use reqwest;
use serde_json::{Value, json};
use os_info::{self, Type, Version};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::sync::LazyLock;
use parking_lot::Mutex;
use terminal_link::Link;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// static STDOUT: LazyLock<Mutex<StandardStream>> = LazyLock::new(|| Mutex::new(StandardStream::stdout(ColorChoice::Always)));
// fn pr<T: std::fmt::Display>(msg: T, col: Color) {
// 	let mut stdout = STDOUT.lock();
// 	stdout.set_color(ColorSpec::new().set_fg(Some(col))).unwrap();
// 	write!(stdout, "{msg}").unwrap();
// }

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
	let info = os_info::get();
	if info.os_type() != Type::Macos {
		panic!("Currently, only MacOS is supported");
	}
	let mut version = String::from(if info.architecture() == Some("arm64") { "arm64_"} else {""});
	version.push_str(match info.version() {
		Version::Semantic(maj, min, _) => {
			match maj {
				..=9 => panic!("Too low"),
				10 => match min {
					..=9 => panic!("Too low"),
					10 => "yosemite",
					11 => "el_capitan",
					12 => "sierra",
					13 => "high_sierra",
					14 => "mojave",
					15 => "catalina",
					_ => panic!("invalid MacOS 10.x version"),
				},
				11 => "big_sur",
				12 => "monterey",
				13 => "ventura",
				14 => "ventura", //TODO: Change
				_ => panic!("Type of gal who runs random tools on pre-alpha macos")
			}
		},
		_ => panic!("Unsupported version schema")
	});

    let args: Vec<String> = std::env::args().collect();

    let all = formula::Formulae::get().await;
	let all_installed = cellar::Cellar::all_installed();

    // match &args.get(1).map(|s| s.as_str()) {
    //     Some("search") => {
    //         let query = args.get(2).expect("a search query");
	// 		let search_results = all.search(query);
	// 		let should_link = true;

	// 		// Closure
	// 		let name = |f: &formula::Formula| f.details.get("name").expect("a name field").as_str().expect("the name to be a valid str").to_string();
			

	// 		let mut stdout = StandardStream::stdout(ColorChoice::Always);
	// 		// Max name length from both exact and near
	// 		let name_max_length = search_results.exact.as_ref().map(|f| name(f).len()).unwrap_or(0).max(search_results.near.iter().map(|f| name(f).len()).max().unwrap_or(0));
	// 		let tap_max_length = search_results.exact.as_ref().map(|f| f.details.get("tap").expect("a tap").as_str().expect("the tap name to be a valid str").len()).unwrap_or(0).max(search_results.near.iter().map(|f| f.details.get("tap").expect("a tap").as_str().expect("the tap name to be a valid str").len()).max().unwrap_or(0));
	// 		let version_max_length = search_results.exact.as_ref().map(|f| f.details.get("versions").expect("a versions key").get("stable").expect("a versions.stable key").as_str().expect("the stable version to be a valid str").len()).unwrap_or(0).max(search_results.near.iter().map(|f| f.details.get("versions").expect("a versions key").get("stable").expect("a versions.stable key").as_str().expect("the stable version to be a valid str").len()).max().unwrap_or(0));
	// 		let desc_max_length = search_results.exact.as_ref().map(|f| f.details.get("desc").expect("a desc field").as_str().expect("the desc to be a valid str").len()).unwrap_or(0).max(search_results.near.iter().map(|f| f.details.get("desc").expect("a desc field").as_str().expect("the desc to be a valid str").len()).max().unwrap_or(0));
		

	// 		// Print near matches
	// 		for formula in search_results.near {
	// 			let name = name(&formula);
	// 			let details = &formula.details;
	// 			let tap = details.get("tap").expect("a tap").as_str().expect("the tap name to be a valid str");
	// 			let formula_version = details.get("versions").expect("a versions key").get("stable").expect("a versions.stable key").as_str().expect("the stable version to be a valid str");
	// 			let desc = details.get("desc").expect("a desc field").as_str().expect("the desc to be a valid str");
	// 			let homepage = details.get("homepage").expect("a homepage field").as_str().expect("the homepage to be a valid str");
	// 			let aliases = details.get("aliases").expect("aliases").as_array().unwrap();

	// 			stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
	// 			write!(stdout, "{:<name_max_length$} ", name).unwrap();

	// 			stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
	// 			write!(stdout, "{:<version_max_length$} ", formula_version).unwrap();

	// 			stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
	// 			write!(stdout, "{:<tap_max_length$} ", tap).unwrap();

	// 			stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
	// 			writeln!(stdout, "{}", Link::new(desc, homepage)).unwrap();
	// 		}
	// 	},
    //     _ => {
    //         panic!("Unknown verb");
    //     }
    // }

	Ok(())
}
