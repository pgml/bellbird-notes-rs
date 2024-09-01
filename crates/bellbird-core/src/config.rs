use std::fs;
use std::path::{Path, PathBuf};

use configparser::ini::Ini;
use ::directories::BaseDirs;
use regex::Regex;
use anyhow::Result;

pub enum ConfigSections {
	General,
	SideBar,
	NotesList,
	BreadCrumb,
	Menu,
}

impl ConfigSections {
	fn as_str(&self) -> &str {
		match self {
			ConfigSections::General => "General",
			ConfigSections::SideBar => "SideBar",
			ConfigSections::NotesList => "NotesList",
			ConfigSections::BreadCrumb => "BreadCrumb",
			ConfigSections::Menu => "Menu",
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

impl ConfigOptions {
	fn as_str(&self) -> &str {
		match self {
			ConfigOptions::DefaultNotesDirectory => "DefaultNotesDirectory",
			ConfigOptions::UserNotesDirectory => "UserNotesDirectory",
			ConfigOptions::DefaultFontSize => "DefaultFontSize",
			ConfigOptions::CurrentDirectory => "CurrentDirectory",
			ConfigOptions::CurrentNote => "CurrentNote",
			ConfigOptions::OpenNotes => "OpenNotes",
			ConfigOptions::Visible => "Visible",
			ConfigOptions::Width => "Width",
		}
	}
}

#[derive(Debug, Clone)]
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
			id = format!("{}-{}", id, "snapshot");
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

	fn config_dir(&self) -> Result<PathBuf> {
		if let Some(base_dirs) = BaseDirs::new() {
			let os_config_dir = base_dirs.config_dir().display().to_string();
			let app_config_dir = self.application_directory_name();
			return Ok(PathBuf::from(&format!("{os_config_dir}/{app_config_dir}")))
		}
		Err(anyhow::anyhow!("Could not find config directory."))
	}

	pub fn config_file(&self, is_meta_info: bool) -> Result<PathBuf, anyhow::Error> {
		match self.config_dir() {
			Ok(config_dir) => {
				if Path::new(&config_dir).is_dir() == false {
					return Err(anyhow::anyhow!(
						"Could not find config directory: {}",
						config_dir.display().to_string()
					));
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
				return Ok(file_path)
			},
			Err(e) => Err(e)
		}
	}

	//pub fn meta_infos_file(&self) ->

	pub fn value(
		&self,
		section: ConfigSections,
		option: ConfigOptions,
		//is_meta_info: bool,
		//file: &str
	) -> Option<String> {
		if let Ok(config_file) = self.config_file(false) {
			let is_file = fs::metadata(&config_file)
				.expect("Couldn't read...")
				.is_file();

			if !Path::new(&config_file).exists() && !is_file {
				return None
			}

			let mut config = Ini::new_cs();
			let mut config_value = String::new();

			if let Ok(_) = config.load(&config_file) {
				if let Some(value) = config.get(&section.as_str(), &option.as_str()) {
					config_value = value.to_string();
				}
			}
			return Some(config_value)
		}
		None
	}

	pub fn set_value(
		&self,
		section: ConfigSections,
		option: ConfigOptions,
		value: String
	) -> Result<()> {
		let mut config = Ini::new_cs();
		let config_file = self.config_file(false)?;
		let _ = config.load(&config_file);
		// read the existing config because it sometimes gets truncated
		let outstring = config.writes();
		let _ = config.read(outstring);
		config.set(section.as_str(), option.as_str(), Some(value.clone()));
		let _ = config.write(&config_file);
		Ok(())
	}
}
