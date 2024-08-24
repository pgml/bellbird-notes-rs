use std::{fs, path::Path};
use anyhow::Result;

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

impl Notes {
	pub fn list(path: &str) -> Result<Vec<Note>> {
		let mut notes = Vec::new();

		if !Path::new(path).exists() {
			return Ok(notes);
		}

		match fs::read_dir(path) {
			Ok(paths) => {
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
			},
			Err(_e) => return Ok(vec![]),
		}

		notes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
		Ok(notes.into())
	}

	pub fn write_to_file(path: String, content: String) -> bool {
		match fs::write(path, content) {
			Ok(_) => true,
			Err(_e) => false,
		}
	}
}

