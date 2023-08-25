use crate::formula::formula::{Formula, ArchivedFormula};
use rkyv::ser::{Serializer, serializers::AllocSerializer};
use rkyv::vec::ArchivedVec;

const FORMULA_URL: &str = "https://formulae.brew.sh/api/formula.json";
const FORMULA_RKYV_FILENAME: &str = "formulae";

pub struct Formulae {
	bytes: Vec<u8>,
}

impl Formulae {
	pub fn get() -> std::io::Result<Self> {
		let bytes = std::fs::read(FORMULA_RKYV_FILENAME)?;

		Ok(Self { bytes })
	}

	pub fn get_archive(&self) -> &ArchivedVec<ArchivedFormula> {
		unsafe { rkyv::archived_root::<Vec<Formula>>(&self.bytes) }
	}

	pub async fn refresh() -> Result<(), Box<dyn std::error::Error>> {
		let json: Vec<Formula> = reqwest::get(FORMULA_URL)
			.await?
			.json()
			.await?;

		let mut serializer = AllocSerializer::<0>::default();
		let _ = serializer.serialize_value(&json)?;
		let bytes = serializer.into_serializer().into_inner();
		std::fs::write(FORMULA_RKYV_FILENAME, bytes)?;
	
		Ok(())
	}

	pub fn search(&self, query: &String) -> FormulaeSearchResult {
		let mut exact: Option<&ArchivedFormula> = None;
		let mut near: Vec<&ArchivedFormula> = Vec::new();

		for f in self.get_archive().iter() {
			if f.name == *query {
				exact = Some(f);
			} else if f.name.contains(query) || f.aliases.iter().any(|s| s.as_str() == query) { // || f.aliases.contains(query) {
				near.push(f);
			}
		}

		FormulaeSearchResult { exact, near }
	}
}

pub struct FormulaeSearchResult<'a> {
	pub exact: Option<&'a ArchivedFormula>,
	pub near: Vec<&'a ArchivedFormula>,
}