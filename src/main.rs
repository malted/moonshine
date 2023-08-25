#![feature(lazy_cell)]

mod cask;
mod formula;
mod cellar;
mod platform;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use terminal_link::Link;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    match &args.get(1).map(|s| s.as_str()) {
        Some("search") => {
			let query = args.get(2).expect("a search query");

			let version = platform::version();
			let all = formula::formulae::Formulae::get().unwrap();
			let all_installed = cellar::Cellar::all_installed();

			let search_results = all.search(query);
			
			let mut stdout = StandardStream::stdout(ColorChoice::Always);

			fn max_length<T, F>(exact: &Option<T>, near: &Vec<T>, extractor: F) -> usize where F: Fn(&T) -> usize, {
				let exact_length = exact.as_ref().map(&extractor).unwrap_or(0);
				let near_length = near.iter().map(&extractor).max().unwrap_or(0);
				exact_length.max(near_length)
			}
			let name_max_length = max_length(&search_results.exact, &search_results.near, |f| f.name.len());
			let tap_max_length = max_length(&search_results.exact, &search_results.near, |f| f.tap.len());
			let desc_max_length = max_length(&search_results.exact, &search_results.near, |f| f.desc.len());

			for formula in search_results.exact.iter().chain(search_results.near.iter()) {
				let installed = all_installed.iter().any(|installed| *installed == formula.name);
				stdout.set_color(ColorSpec::new().set_fg(None).set_bold(installed))?;
				write!(stdout, "{}", formula.name)?;
				
				let adjusted_pad = name_max_length - formula.name.len();
				if installed {
					stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
					write!(stdout, " {:<adjusted_pad$}", "✔")?;
				} else {
					write!(stdout, "{:<adjusted_pad$} ", "")?;
				}

				// let supported = if formula.bottle.stable.files.get(&version).is_some() { "  " } else { "🚫🍾" };
				// write!(stdout, "{:<2} ", supported)?;

				match formula.tap.as_str() {
					"homebrew/core" => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?,
					_ => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?,
				}
				write!(stdout, "{:<tap_max_length$} ", formula.tap)?;

				stdout.set_color(ColorSpec::new().set_fg(None))?;
				writeln!(stdout, "{}", Link::new(&formula.desc, &formula.homepage))?;
			}
		},
		Some("refresh") => {
			formula::formulae::Formulae::refresh().await?;
		},
        _ => {
            panic!("Unknown verb");
        }
    }

	Ok(())
}
