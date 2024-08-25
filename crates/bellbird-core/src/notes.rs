use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

use crate::config::{Config, ConfigOptions, ConfigSections};

#[derive(Debug)]
pub struct Note {
	// pub id: i32,
	pub name: String,
	pub	path: String,
	// pub	preview: String,
	// pub is_pinned: bool,
	// pub is_current: bool,
	// pub is_loaded: bool,
	// pub creation_date: String,
	// pub file_size: u64
}

#[derive(Debug)]
pub struct Notes;

impl<'a> Notes {
	pub fn list(path: &Path) -> Result<Vec<Note>> {
		let mut notes = Vec::new();

		if !Path::new(path).exists() {
			return Ok(notes);
		}

		if let Ok(paths) = fs::read_dir(path) {
			for path in paths {
				let dir_entry = path?;

				if dir_entry.path().is_dir() {
					continue
				}

				let file_path = dir_entry.path().display().to_string();
				let mut file_name = dir_entry.file_name();

				file_name = Path::new(&file_name).with_extension("").into();

				notes.push(Note {
					name: file_name.to_str().unwrap().to_string(),
					path: file_path.clone(),
				});
			}
		}

		notes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
		Ok(notes.into())
	}

	pub fn write_to_file(path: &Path, content: String) -> bool {
		match fs::write(path, content) {
			Ok(_) => true,
			Err(_) => false,
		}
	}

	pub fn current_note_path() -> PathBuf {
		let config = Config::new();
		let mut value = config.value(
			ConfigSections::General,
			ConfigOptions::CurrentNote
		);
		value = value.replace("file://", "");
		PathBuf::from(value)
	}

	pub fn set_current_note_path(path: &Path) {
		Config::new().set_value(
			ConfigSections::General,
			ConfigOptions::CurrentNote,
			path.display().to_string()
		);
	}
}
