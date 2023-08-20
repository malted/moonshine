#![feature(lazy_cell)]

mod cask;
mod formula;
mod cellar;

use formula::formula::Formula;
use formula::formulae::FormulaeSearchResult;

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

    let all = formula::formulae::Formulae::get().await.unwrap();
	let all_installed = cellar::Cellar::all_installed();

    match &args.get(1).map(|s| s.as_str()) {
        Some("search") => {
            let query = args.get(2).expect("a search query");
			let search_results = all.search(query);
			let should_link = true;
			
			let mut stdout = StandardStream::stdout(ColorChoice::Always);

			// fn max_length<'a, F>(search_result: &'a FormulaeSearchResult, extractor: F) -> usize
			// where
			// 	F: FnMut(&'a Formula) -> &str
			// {
			// 	search_result.exact.iter().chain(&search_result.near).map(extractor).map(|s| s.len()).max().unwrap_or(0)
			// }

			// let name_max_length = max_length(&search_results, |f| &f.name);
			// let tap_max_length = max_length(&search_results, |f| &f.tap);
			// let version_max_length = max_length(&search_results, |f| &f.version); // let version_max_length = search_results.exact.as_ref().map(|f| f.details.get("versions").expect("a versions key").get("stable").expect("a versions.stable key").as_str().expect("the stable version to be a valid str").len()).unwrap_or(0).max(search_results.near.iter().map(|f| f.details.get("versions").expect("a versions key").get("stable").expect("a versions.stable key").as_str().expect("the stable version to be a valid str").len()).max().unwrap_or(0));
			// let desc_max_length = max_length(&search_results, |f| &f.desc);
		
			let name_max_length = search_results.exact.as_ref().map(|f| f.name.len()).unwrap_or(0).max(search_results.near.iter().map(|f| f.name.len()).max().unwrap_or(0));
			let tap_max_length = search_results.exact.as_ref().map(|f| f.tap.len()).unwrap_or(0).max(search_results.near.iter().map(|f| f.tap.len()).max().unwrap_or(0));
			let desc_max_length = search_results.exact.as_ref().map(|f| f.desc.len()).unwrap_or(0).max(search_results.near.iter().map(|f| f.desc.len()).max().unwrap_or(0));

			// Print near matches
			for formula in search_results.near {
				stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
				write!(stdout, "{:<name_max_length$} ", formula.name).unwrap();

				stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
				write!(stdout, "{:<tap_max_length$} ", formula.tap).unwrap();

				stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
				writeln!(stdout, "{}", Link::new(&formula.desc, &formula.homepage)).unwrap();
			}
		},
        _ => {
            panic!("Unknown verb");
        }
    }

	Ok(())
}
