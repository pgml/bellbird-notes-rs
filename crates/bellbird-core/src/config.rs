//use async_std::path::PathBuf;
use std::path::PathBuf;

use configparser::ini::Ini;
use ::directories::BaseDirs;
use regex::Regex;
use anyhow::Result;

#[derive(Debug)]
pub enum ConfigSections {
	General,
	SideBar,
	NotesList,
	BreadCrumb,
	Menu,
}

impl ConfigSections {
	pub fn as_str(&self) -> &str {
		match self {
			ConfigSections::General => "General",
			ConfigSections::SideBar => "SideBar",
			ConfigSections::NotesList => "NotesList",
			ConfigSections::BreadCrumb => "BreadCrumb",
			ConfigSections::Menu => "Menu",
		}
	}
}

#[derive(Debug)]
pub enum ConfigOptions {
	DefaultNotesDirectory,
	UserNotesDirectory,
	DefaultFontSize,
	CurrentDirectory,
	CurrentNote,
	OpenNotes,
	Visible,
	Width,
	CaretPosition,
	Pinned,
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
			ConfigOptions::CaretPosition => "CaretPosition",
			ConfigOptions::Pinned => "Pinned",
		}
	}
}

#[derive(Debug, Clone)]
pub struct Config {
	ini: Ini,
	ini_file: PathBuf,
}

impl<'a> Config {
	pub fn new() -> Self {
		let ini = Ini::new_cs();

		Self {
			ini,
			ini_file: PathBuf::new(),
		}
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
				if std::path::Path::new(&config_dir).is_dir() == false {
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

	pub fn meta_info(
		&self,
		section: &str,
		option: ConfigOptions,
	) -> Option<String> {
		self.value(section, option, true)
	}

	fn load_file_async(&mut self, is_meta_info: bool) {
		self.ini_file = self.config_file(is_meta_info).unwrap();
		let _ = self.ini.load_async(&self.ini_file);
	}

	fn load_file(&mut self, is_meta_info: bool) {
		self.ini_file = self.config_file(is_meta_info).unwrap();
		let _ = self.ini.load(&self.ini_file);
	}

	pub fn config_value(
		&self,
		section: &str,
		option: ConfigOptions,
	) -> Option<String> {
		self.value(section, option, false)
	}

	fn value(
		&self,
		section: &str,
		option: ConfigOptions,
		is_meta_info: bool,
		//file: &str
	) -> Option<String> {
		if let Ok(config_file) = self.config_file(is_meta_info) {
			if !config_file.exists() && !config_file.is_file() {
				return None
			}

			let mut config = Ini::new_cs();
			let mut config_value = None;

			if let Ok(_) = config.load(&config_file) {
				if let Some(value) = config.get(&section, &option.as_str()) {
					config_value = Some(value.to_string());
				}
			}
			return Some(config_value)?
		}
		None
	}

	pub fn sections_by_value(
		&mut self,
		option: ConfigOptions,
		expected_value: String
	) -> Option<Vec<String>> {
		self.load_file(true);

		let mut sections = vec![];
		for section in self.ini.sections().iter() {
			let value = self.ini.get(&section, &option.as_str());
			if value.is_some() && !expected_value.is_empty() {
				if value.unwrap() == expected_value {
					sections.push(section.to_string());
				}
			}
		}

		return Some(sections);
	}

	pub async fn set_config_value_async(
		&mut self,
		section: &str,
		option: ConfigOptions,
		value: String,
	) -> Result<()> {
		self.set_value_async(section, option, value, false).await
	}

	pub fn set_config_value(
		&mut self,
		section: &str,
		option: ConfigOptions,
		value: String,
	) -> Result<()> {
		self.set_value(section, option, value, false)
	}

	pub async fn set_meta_value_async(
		&mut self,
		section: &str,
		option: ConfigOptions,
		value: String,
	) -> Result<()> {
		self.set_value_async(section, option, value, true).await
	}

	pub fn set_meta_value(
		&mut self,
		section: &str,
		option: ConfigOptions,
		value: String,
	) -> Result<()> {
		self.set_value(section, option, value, true)
	}

	async fn set_value_async(
		&mut self,
		section: &str,
		option: ConfigOptions,
		value: String,
		is_meta_info: bool
	) -> Result<()> {
		self.load_file_async(is_meta_info);
		// read the existing config because it sometimes gets truncated
		let outstring = self.ini.writes();
		let _ = self.ini.read(outstring);
		self.ini.set(section, option.as_str(), Some(value.clone()));
		let _ = self.ini.write_async(&self.ini_file).await;
		Ok(())
	}

	fn set_value(
		&mut self,
		section: &str,
		option: ConfigOptions,
		value: String,
		is_meta_info: bool
	) -> Result<()> {
		self.load_file(is_meta_info);
		// read the existing config because it sometimes gets truncated
		let outstring = self.ini.writes();
		let _ = self.ini.read(outstring);
		self.ini.set(section, option.as_str(), Some(value.clone()));
		let _ = self.ini.write(&self.ini_file);
		Ok(())
	}
}
