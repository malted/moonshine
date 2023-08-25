#![allow(dead_code)]

use os_info::{Type, Version};

pub fn version() -> String {
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

	version
}