use serde::{Deserialize, Serialize};
// use simd_json::owned::Value;

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Formula {
    conflicts_with: Vec<String>,
    pub homepage: String,
    installed: Vec<Installed>,
    deprecation_reason: Option<String>,
    options: Vec<String>,
    recommended_dependencies: Vec<String>,
    keg_only: bool,
    pub desc: String,
    ruby_source_checksum: Checksum,
    pub name: String,
    build_dependencies: Vec<String>,
    urls: Urls,
    // uses_from_macos_bounds: Vec<Value>,
    // uses_from_macos: Vec<Value>,
    disable_date: Option<String>,
    deprecated: bool,
    disabled: bool,
    ruby_source_path: String,
    pub tap: String,
    variations: Variations,
    requirements: Vec<Requirement>,
    oldname: Option<String>,
    test_dependencies: Vec<String>,
    disable_reason: Option<String>,
    deprecation_date: Option<String>,
    link_overwrite: Vec<String>,
    conflicts_with_reasons: Vec<Option<String>>,
    pinned: bool,
    post_install_defined: bool,
    optional_dependencies: Vec<String>,
    versioned_formulae: Vec<String>,
    pub bottle: Bottle,
    pub versions: Versions,
    service: Option<Service>,
    keg_only_reason: Option<KegOnlyReason>,
    revision: i64,
    dependencies: Vec<String>,
    linked_keg: Option<String>,
    outdated: bool,
    full_name: String,
    tap_git_head: String,
    pub aliases: Vec<String>,
    version_scheme: i64,
    oldnames: Vec<String>,
    caveats: Option<String>,
    license: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Checksum {
    sha256: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Urls {
    head: Option<UrlDetails>,
    stable: UrlDetails,
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct UrlDetails {
    branch: Option<String>,
    url: Option<String>,
    checksum: Option<String>,
    revision: Option<String>,
    tag: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Variations {}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Bottle {
    pub stable: BottleStable,
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct BottleStable {
    pub files: BottleFiles,
    rebuild: i64,
    root_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct BottleFiles {
	all: Option<FileDetails>,
	x86_64_linux: Option<FileDetails>,
	el_capitan: Option<FileDetails>,
	sierra: Option<FileDetails>,
	high_sierra: Option<FileDetails>,
	mojave: Option<FileDetails>,
	catalina: Option<FileDetails>,
    big_sur: Option<FileDetails>,
	arm64_big_sur: Option<FileDetails>,
	monterey: Option<FileDetails>,
	arm64_monterey: Option<FileDetails>,
    ventura: Option<FileDetails>,
	arm64_ventura: Option<FileDetails>,
	// sonoma: Option<FileDetails>, //TODO: Uncomment
	// arm64_sonoma: Option<FileDetails>, //TODO: Uncomment
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct FileDetails {
    cellar: Option<String>,
    sha256: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Versions {
    bottle: bool,
    head: Option<String>,
    pub stable: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Requirement {
	cask: Option<String>,
	contexts: Vec<String>,
	name: String,
	specs: Vec<String>,
	version: Option<String>,
	// download is always null
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Installed {
	built_as_bottle: bool,
	installed_as_dependency: bool,
	installed_on_request: bool,
	poured_from_bottle: bool,
	runtime_dependencies: Vec<InstalledRuntimeDependency>,
	time: i64,
	version: String,
	// used_options is always an empty array
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct InstalledRuntimeDependency {
	declared_directly: bool,
	full_name: String,
	version: String,
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Service {
	keep_alive: Option<ServiceKeepAlive>,
	// run: Option<Value>, // Either {linux: Vec<String>, macos: Vec<String>} or Vec<String>
	run_type: Option<String>,
	working_dir: Option<String>,
	require_root: Option<bool>,
	log_path: Option<String>,
	error_log_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct ServiceKeepAlive {
	always: Option<bool>,
	crashed: Option<bool>,
	successful_exit: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct KegOnlyReason {
	explanation: String,
	reason: String,
}