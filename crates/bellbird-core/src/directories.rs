use std::{fs, path::{Path, PathBuf}};
use anyhow::Result;

use directories::UserDirs;
use walkdir::WalkDir;

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
	pub fn list(path: &Path, max_depth: usize) -> Result<Vec<Directory>> {
		let mut directories: Vec<Directory> = vec![];
		let walk_dir_iter = WalkDir::new(path)
			.min_depth(1)
			.max_depth(max_depth);

		for entry in walk_dir_iter {
			let entry_clone = entry?.clone();
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

		directories.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

		Ok(directories)
	}

	pub fn root_directory() -> PathBuf {
		let mut root_dir = PathBuf::from(Self::home_directory());

		let config = Config::new();
		let value = config.value(
			ConfigSections::General,
			ConfigOptions::UserNotesDirectory
		);

		root_dir.push(value);
		root_dir
	}

	fn home_directory() -> PathBuf {
		match UserDirs::new() {
			Some(user_dirs) => {
				user_dirs.home_dir().to_path_buf()
			},
			_ => PathBuf::new()
		}
	}

	pub fn current_directory_path() -> PathBuf {
		let config = Config::new();
		let value = config.value(
			ConfigSections::General,
			ConfigOptions::CurrentDirectory
		);
		PathBuf::from(value)
	}

	pub fn set_current_directory_path(path: &Path) {
		Config::new().set_value(
			ConfigSections::General,
			ConfigOptions::CurrentDirectory,
			path.display().to_string()
		);
	}

	pub fn get_depth_from_root(path: &Path) -> u32 {
		let root_dir = Self::root_directory();
		let walk_dir_iter = WalkDir::new(root_dir)
			.min_depth(1)
			.max_depth(20);

		for entry in walk_dir_iter {
			if let Ok(dir) = entry {
				if path == dir.path() {
					return (dir.depth() - 1) as u32;
				}
			}
		}
		0
	}

	pub fn dir_has_children(path: &Path) -> bool {
		if path.exists() {
			Self::list(path, 1).unwrap().len() > 0
		}
		else {
			false
		}
	}

	pub fn create(path: &Path) -> bool
	{
		if path.exists() {
			return false
		}

		match fs::create_dir(path) {
			Ok(_) => true,
			Err(e) => {
				println!("Couldn't create folder `{:?}` - {}", path, e.to_string());
				false
			},
		}
	}

	pub fn rename(old_path: &Path, new_path: &Path) -> bool {
		return match fs::rename(old_path, new_path) {
			Ok(()) => true,
			Err(e) => {
				println!("{}", e);
				false
			}
		}
	}

	pub fn delete(path: &Path, mut delete_files: bool) -> bool
	{
		if !delete_files {
			delete_files = false
		}

		//remove_section(&path, true);

		if !delete_files {
			match fs::remove_dir(path) {
				Ok(_) => true,
				Err(_e) => false,
			}
		}
		else {
			match fs::remove_dir_all(path) {
				Ok(_) => true,
				Err(_e) => false,
			}
		}
	}
}
