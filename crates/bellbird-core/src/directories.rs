use std::fs;
use std::path::{Path, PathBuf};
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
	pub fn list(path: &Path, recursive: bool) -> Result<Vec<Directory>> {
		let mut directories: Vec<Directory> = vec![];

		if !path.is_dir() {
			return Ok(directories);
		}

		for directory in fs::read_dir(path)? {
			let path_buf = directory.as_ref().unwrap().path();
			let file_name = path_buf.file_name().unwrap().to_str().unwrap().to_string();
			//let path_str = path_buf;

			if path_buf.is_dir() {
				directories.push(Directory {
					name: file_name,
					path: path_buf.clone(),
					children: if recursive {
						Self::list(&path_buf, true)?
					}
					else {
						vec![]
					},
					//nbr_notes: 0,
					//nbr_folders: 0,
					//is_expanded: false,
				});
			}
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
		for entry in WalkDir::new(root_dir).max_depth(20) {
			if let Ok(dir) = entry {
				if path == dir.path() {
					return (dir.depth() - 1) as u32;
				}
			}
		}
		0
	}

	pub fn dir_has_children(path: &Path) -> bool {
		// @todo don't use Self::list
		if let Ok(directory) = Self::list(path, true) {
			for dir in directory {
				return dir.children.len() >= 0;
			}
		}
		false
	}
}
