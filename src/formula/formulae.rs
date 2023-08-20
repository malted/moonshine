use simd_json::{owned::Value, ValueAccess};
use crate::formula::formula::Formula;

const FORMULA_URL: &str = "https://formulae.brew.sh/api/formula.json";
const MS_API_URL: &str = "https://moonshine.malted.dev/api/v1/formula_hash";
const UP_TO_DATE_FORMULA_HASH_FILENAME: &str = "formula-hash";
const FORMULA_JSON_FILENAME: &str = "formula.json";

#[derive(Debug)]
pub struct Formulae (Vec<Formula>);
impl Formulae {
	pub async fn get() -> Result<Self, Box<dyn std::error::Error>> {
		let mut json_str = std::fs::read_to_string(FORMULA_JSON_FILENAME)?;
		let parsed: Vec<Formula> = unsafe { simd_json::from_str(&mut json_str).unwrap() };
	
		Ok(Self(parsed))
	}

	// async fn get_json() -> Result<Value, Box<dyn Error>> {
	// 	let utd_hash = "844f12ea69bcbaa6863a108e740d8371ad9f4337ae9b1521da2b7e3656cb54af";

	// 	if match std::fs::read_to_string(UP_TO_DATE_FORMULA_HASH_FILENAME) {
	// 		Ok(hash) => utd_hash == hash,
	// 		Err(_) => false,
	// 	} {
	// 		// Read the stored JSON
	// 		let mut json_str = std::fs::read_to_string(FORMULA_JSON_FILENAME)?;
	// 		let parsed = unsafe { simd_json::from_str(&mut json_str).unwrap() };

	// 		Ok(parsed)
	// 	} else {
	// 		// Fetch the up-to-date JSON
	// 		let response = reqwest::get(FORMULA_URL).await?;

	// 		// Convert the response into a JSON object
	// 		let json: Value = response.json().await?;
	// 		let json_str = json.to_string();

	// 		// Remove them both right before writing
	// 		std::fs::remove_file(FORMULA_JSON_FILENAME).unwrap_or(());
	// 		std::fs::remove_file(UP_TO_DATE_FORMULA_HASH_FILENAME).unwrap_or(());

	// 		std::fs::write(FORMULA_JSON_FILENAME, &json_str)?;

	// 		// Hash it
	// 		let mut hasher = Sha256::new();
	// 		hasher.update(json_str);
	// 		let hash = hasher.finalize();

	// 		// Save the hashed bytes to a file
	// 		std::fs::write("formula-hash", format!("{:#x}", hash))?;
			
	// 		Ok(json)
	// 	}
	// }

	pub fn search(&self, query: &String) -> FormulaeSearchResult {		
		let mut exact: Option<&Formula> = None;
		let mut near: Vec<&Formula> = Vec::new();

		for f in self.0.iter() {
			if f.name == *query {
				exact = Some(f);
			} else if f.name.contains(query) || f.aliases.contains(query) {
				near.push(f);
			}
		}

		FormulaeSearchResult { exact, near }
	}
}

#[derive(Debug)]
pub struct FormulaeSearchResult<'a> {
	pub exact: Option<&'a Formula>,
	pub near: Vec<&'a Formula>,
}