use async_std::{fs, stream::StreamExt};
use std::path::{Path, PathBuf};
use anyhow::Result;

use crate::config::{Config, ConfigOptions, ConfigSections};

const NOTES_EXTENSION: &str = "note";

#[derive(Debug, Clone)]
pub struct Note {
	// pub id: i32,
	pub name: String,
	pub	path: String,
	// pub	preview: String,
	pub is_pinned: bool,
	// pub is_current: bool,
	// pub is_loaded: bool,
	// pub creation_date: String,
	// pub file_size: u64
}

#[derive(Debug)]
pub struct Notes;

impl<'a> Notes {
	pub async fn list(path: &Path) -> Result<Vec<Note>> {
		if !path.exists() {
			return Err(anyhow::anyhow!(
				"Could not list file, path does not exist: {}",
				path.display().to_string()
			));
		}

		let pinned_notes = Notes::pinned_notes(path).unwrap();
		let mut notes: Vec<Note> = vec![];

		if let Ok(mut paths) = fs::read_dir(path).await {
			while let Some(path) = paths.next().await {
				if let Ok(dir_entry) = path {
					if dir_entry.path().is_dir().await {
						continue
					}

					let file_path = dir_entry.path().display().to_string();
					let mut file_name = dir_entry.file_name();

					let mut is_pinned = false;
					if pinned_notes.contains(&file_path) {
						is_pinned = true;
					}

					file_name = Path::new(&file_name).with_extension("").into();

					notes.push(Note {
						name: file_name.to_str().unwrap().to_string(),
						path: file_path.clone(),
						is_pinned
					});
				}
			}
		}

		notes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
		Ok(notes)
	}

	pub fn write_to_file(mut path: PathBuf, content: String) -> Result<()> {
		path = Self::ensure_correct_path(&path);
		let _ = std::fs::write(path, content);
		Ok(())
	}

	pub async fn rename(mut old_path: PathBuf, mut new_path: PathBuf) -> Result<bool> {
		old_path = Self::ensure_correct_path(&old_path);
		new_path = Self::ensure_correct_path(&new_path);

		return match fs::rename(old_path, new_path).await {
			Ok(()) => Ok(true),
			Err(e) => Err(anyhow::anyhow!("Could not rename file: {}", e))
		}
	}

	pub async fn delete(path: &Path) -> bool {
		match fs::remove_file(path).await {
			Ok(_) => true,
			Err(_e) => false,
		}
	}

	pub fn current_path() -> Option<PathBuf> {
		let config = Config::new();
		match config.config_value(
			ConfigSections::General.as_str(),
			ConfigOptions::CurrentNote
		) {
			Some(value) => {
				let current_note = value.replace("file://", "");
				Some(PathBuf::from(current_note))
			},
			_ => None
		}
	}

	pub fn set_current_path(path: &Path) {
		let _  = Config::new().set_config_value(
			ConfigSections::General.as_str(),
			ConfigOptions::CurrentNote,
			path.display().to_string()
		);
	}

	fn ensure_correct_path(path: &Path) -> PathBuf {
		match path.extension() {
			Some(_) => path.to_path_buf(),
			None => {
				let path_with_extension = format!(
					"{}.{}",
					path.to_str().unwrap(),
					NOTES_EXTENSION
				);
				PathBuf::from(path_with_extension)
			}
		}
	}

	pub fn is_pinned(path: &std::path::Path) -> bool {
		let config = Config::new();
		let path = path.display().to_string();
		match config.meta_info(&path, ConfigOptions::Pinned) {
			Some(value) => value == "true",
			None => false
		}
	}

	pub fn set_is_pinned(path: &Path, is_pinned: bool) {
		let is_pinned = if is_pinned == true { "true" } else { "false" };
		let _  = Config::new().set_meta_value(
			&path.display().to_string(),
			ConfigOptions::Pinned,
			is_pinned.to_string()
		);
	}

	pub fn pinned_notes(path: &Path) -> Option<Vec<String>> {
		let mut config = Config::new();
		let sections = config.sections_by_value(ConfigOptions::Pinned, "true".to_string());
		let path_str = path.to_str().unwrap();
		let mut pinned_notes = vec![];

		sections.clone().unwrap().iter().for_each(|section| {
			if section.contains(path_str) {
				pinned_notes.push(section.to_string());
			}
		});

		Some(pinned_notes)
	}
}
