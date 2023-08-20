use std::error::Error;
use reqwest;
use simd_json::{owned::Value, ValueAccess};
use sha2::{Sha256, Digest};

const FORMULA_URL: &str = "https://formulae.brew.sh/api/formula.json";
const MS_API_URL: &str = "https://moonshine.malted.dev/api/v1/formula_hash";
const UP_TO_DATE_FORMULA_HASH_FILENAME: &str = "formula-hash";
const FORMULA_JSON_FILENAME: &str = "formula.json";

pub struct Formulae (Vec<Formula>);
pub struct FormulaeSearchResult {
	pub exact: Option<Formula>,
	pub near: Vec<Formula>,
}

pub struct Formula {
	pub details: Value
}

impl Formulae {
	pub async fn get() -> Result<Self, Box<dyn Error>> {
		println!("Fetching formulae...");
		let json = Self::get_json().await?;
		
		let arr = json.as_array().unwrap();

		    let mut formulae = Vec::with_capacity(arr.len());
		for formula in arr {
			formulae.push(Formula { details: formula.clone() });
		}

		Ok(Self(formulae))
	}

	async fn get_json() -> Result<Value, Box<dyn Error>> {
		let utd_hash = "844f12ea69bcbaa6863a108e740d8371ad9f4337ae9b1521da2b7e3656cb54af";

		if match std::fs::read_to_string(UP_TO_DATE_FORMULA_HASH_FILENAME) {
			Ok(hash) => utd_hash == hash,
			Err(_) => false,
		} {
			// Read the stored JSON
			let mut json_str = std::fs::read_to_string(FORMULA_JSON_FILENAME)?;
			let parsed = unsafe { simd_json::from_str(&mut json_str).unwrap() };

			Ok(parsed)
		} else {
			// Fetch the up-to-date JSON
			let response = reqwest::get(FORMULA_URL).await?;

			// Convert the response into a JSON object
			let json: Value = response.json().await?;
			let json_str = json.to_string();

			// Remove them both right before writing
			std::fs::remove_file(FORMULA_JSON_FILENAME).unwrap_or(());
			std::fs::remove_file(UP_TO_DATE_FORMULA_HASH_FILENAME).unwrap_or(());

			std::fs::write(FORMULA_JSON_FILENAME, &json_str)?;

			// Hash it
			let mut hasher = Sha256::new();
			hasher.update(json_str);
			let hash = hasher.finalize();

			// Save the hashed bytes to a file
			std::fs::write("formula-hash", format!("{:#x}", hash))?;
			
			Ok(json)
		}
	}

	pub fn search(&self, query: &str) -> FormulaeSearchResult {		
		let mut exact: Option<Formula> = None;
		let mut near: Vec<Formula> = Vec::new();

		for f in self.0.iter() {
			let f = &f.details;

			let name = f.get("name").expect("a name field").as_str().expect("the name to be a valid str");
			let tap = f.get("tap").expect("a tap").as_str().expect("the tap name to be a valid str");
			let formula_version = f
				.get("versions")
				.and_then(|v| v.get("stable"))
				.and_then(|v| v.as_str())
				.expect("the stable version to be a valid str");
    
			if name == query {
				exact = Some(Formula { details: f.clone() });
			} else if name.contains(query) || f.get("aliases").expect("aliases").as_array().unwrap().contains(&Value::from(query)) {
				near.push(Formula { details: f.clone() });
			}
		}

		FormulaeSearchResult { exact, near }
	}
}
