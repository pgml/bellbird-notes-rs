use std::{fs, path::{Path, PathBuf}};

use directories::UserDirs;
use walkdir::WalkDir;
use anyhow::Result;

use crate::config::{Config, ConfigOptions, ConfigSections};

#[derive(Debug, Clone)]
pub struct Directory {
	pub name: String,
	pub path: PathBuf,
	pub children: Vec<Directory>,
	//pub nbr_notes: usize,
	//pub nbr_folders: usize,
	//pub is_expanded: bool,
}

#[derive(Debug, Clone)]
pub struct Directories;

impl Directories {
	pub fn list(path: &Path, max_depth: usize) -> Option<Vec<Directory>> {
		let mut directories: Vec<Directory> = vec![];
		let walk_dir_iter = WalkDir::new(path)
			.min_depth(1)
			.max_depth(max_depth)
			.into_iter();

		for entry in walk_dir_iter.filter_entry(|e| !Self::is_hidden(e)) {
			let entry_clone = entry.unwrap();
			let path = entry_clone.path();

			if !path.is_dir() {
				continue;
			}

			directories.push(Directory {
				name: path.file_name().unwrap().to_str().unwrap().to_string(),
				path: path.to_path_buf(),
				// get direct children for now
				//children: Self::list(path, 1)?
				children: vec![]
			})
		}

		directories.sort_by(|a, b|
			a.name.to_lowercase().cmp(&b.name.to_lowercase()));

		Some(directories)
	}

	pub fn bb_root_directory() -> Option<PathBuf> {
		if let Some(home_dir) = Self::home_directory() {
			let mut root_dir = PathBuf::from(home_dir);

			let config = Config::new();
			let value = config.config_value(
				ConfigSections::General.as_str(),
				ConfigOptions::UserNotesDirectory,
			)?;

			root_dir.push(value);
			return Some(root_dir);
		}
		None
	}

	fn home_directory() -> Option<PathBuf> {
		match UserDirs::new() {
			Some(user_dirs) => {
				Some(user_dirs.home_dir().to_path_buf())
			},
			_ => None
		}
	}

	pub fn current_directory_path() -> Option<PathBuf> {
		let config = Config::new();
		match config.config_value(
			ConfigSections::General.as_str(),
			ConfigOptions::CurrentDirectory
		) {
			Some(current_dir) => Some(PathBuf::from(current_dir)),
			_ => None
		}
	}

	pub fn set_current_directory_path(path: &Path) {
		let _ = Config::new().set_config_value(
			ConfigSections::General.as_str(),
			ConfigOptions::CurrentDirectory,
			path.display().to_string()
		);
	}

	pub fn get_depth_from_root(path: &Path) -> u32 {
		if let Some(root_dir) = Self::bb_root_directory() {
			let walk_dir_iter = WalkDir::new(root_dir)
				.min_depth(1)
				.max_depth(20)
				.into_iter();

			for entry in walk_dir_iter {
				if let Ok(dir) = entry {
					if path == dir.path() {
						return (dir.depth() - 1) as u32;
					}
				}
			}
		}
		0
	}

	pub fn dir_has_children(path: &Path) -> bool {
		if !path.exists() {
			return false;
		}
		match Self::list(path, 1) {
			Some(entries) => entries.len() > 0,
			None => false
		}
	}

	pub fn create(path: &Path) -> Result<(), std::io::Error> {
		fs::create_dir(path).map_err(|e| {
			eprintln!("Failed to create directory: {:?}", e);
			e
		})
	}

	pub fn rename(old_path: &Path, new_path: &Path) -> Result<(), std::io::Error> {
		fs::rename(old_path, new_path).map_err(|e| {
			eprintln!("Failed to rename directory: {:?}", e);
			e
		})
	}

	pub fn delete(path: &Path, mut delete_files: bool) -> Result<(), std::io::Error> {
		if !delete_files {
			delete_files = false
		}

		//remove_section(&path, true);

		if !delete_files {
			fs::remove_dir(path).map_err(|e| {
				eprintln!("Failed to delete directory: {:?}", e);
				e
			})
		}
		else {
			fs::remove_dir_all(path).map_err(|e| {
				eprintln!("Failed to recursively delete directory: {:?}", e);
				e
			})
		}
	}

	fn is_hidden(entry: &walkdir::DirEntry) -> bool {
		entry.file_name()
			.to_str()
			.map(|s| s.starts_with("."))
			.unwrap_or(false)
	}
}
