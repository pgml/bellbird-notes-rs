use std::{
	ffi::{OsStr, OsString}, fs, path::{Path, PathBuf}
};

use configparser::ini::Ini;
use ::directories::BaseDirs;
use regex::Regex;

pub enum ConfigSections {
	General,
	SideBar,
	NotesList,
	BreadCrumb,
	Menu,
}

impl ToString for ConfigSections {
	fn to_string(&self) -> String {
		match self {
			ConfigSections::General => self.to_string(),
			ConfigSections::SideBar => self.to_string(),
			ConfigSections::NotesList => self.to_string(),
			ConfigSections::BreadCrumb => self.to_string(),
			ConfigSections::Menu => self.to_string(),
		}
	}
}

pub enum ConfigOptions {
	DefaultNotesDirectory,
	UserNotesDirectory,
	DefaultFontSize,
	CurrentDirectory,
	CurrentNote,
	OpenNotes,
	Visible,
	Width,
}

impl ToString for ConfigOptions {
	fn to_string(&self) -> String {
		match self {
			ConfigOptions::DefaultNotesDirectory => self.to_string(),
			ConfigOptions::UserNotesDirectory => self.to_string(),
			ConfigOptions::DefaultFontSize => self.to_string(),
			ConfigOptions::CurrentDirectory => self.to_string(),
			ConfigOptions::CurrentNote => self.to_string(),
			ConfigOptions::OpenNotes => self.to_string(),
			ConfigOptions::Visible => self.to_string(),
			ConfigOptions::Width => self.to_string(),
		}
	}
}

pub struct Config;

impl<'a> Config {
	pub fn new() -> Self {
		Self {}
	}

	pub fn app_version(&self) -> String {
		let mut version = "";

		if cfg!(feature = "stable") {
			version = "stable";
		}

		if cfg!(feature = "snapshot") {
			version = "snapshot";
		}

		version.to_string()
	}

	pub fn app_id(&self) -> String {
		let mut id = "org.bellbird.notes".to_string();
		if self.app_version() == "snapshot".to_string() {
			id = format!("{} {}", id, "-snapshot");
		}
		id
	}

	pub fn app_name(&self) -> String {
		let mut name = "Bellbird Notes".to_string();
		if self.app_version() == "snapshot".to_string() {
			name = format!("{} {}", name, "Snapshot");
		}
		name
	}

	fn application_directory_name(&self) -> String {
		let regex = Regex::new(r"\s+").unwrap();
		let directory_name = regex.replace_all(&self.app_name(), "-").to_string();
		directory_name.to_lowercase()
	}

	fn config_dir(&self) -> PathBuf {
		match BaseDirs::new() {
			Some(base_dirs) => {
				let os_config_dir = base_dirs.config_dir().display().to_string();
				let app_config_dir = self.application_directory_name();
				//OsStr::new(&format!("{os_config_dir}/{app_config_dir}")).into()
				PathBuf::from(&format!("{os_config_dir}/{app_config_dir}"))
			},
			None => PathBuf::new()
		}
	}

	pub fn config_file(&self, is_meta_info: bool) -> PathBuf {
		let binding = self.config_dir();
		let config_dir = binding.as_path();

		if Path::new(&config_dir).is_dir() == false {
			return PathBuf::new();
		}

		let mut filename = self.application_directory_name();

		if is_meta_info {
			filename = format!("{filename}_metainfos");
		}
		else {
			filename = format!("{filename}.conf");
		}

		let mut file_path = PathBuf::from(config_dir);
		file_path.push(&filename);
		file_path
	}

	pub fn value(
		&self,
		section: &str,
		option: &str,
		//is_meta_info: bool,
		//file: &str
	) -> String {
		let config_file = self.config_file(false);

		let is_file = fs::metadata(&config_file)
			.expect("Couldn't read...")
			.is_file();

		if !Path::new(&config_file).exists() && !is_file {
			return String::new();
		}

		let mut config = Ini::new_cs();
		let mut config_value = String::new();

		if let Ok(_) = config.load(&config_file) {
			if let Some(value) = config.get(section, option) {
				config_value = value.to_string();
			}
		}
		config_value
	}
}
