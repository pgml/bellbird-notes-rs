pub(crate) mod imp;

use std::{fmt::write, path::Path};

use bellbird_core::directories::Directories;
use gtk::{glib, subclass::prelude::*};

glib::wrapper! {
	pub struct Breadcrumb(ObjectSubclass<imp::Breadcrumb>)
		@extends gtk::Widget, gtk::Box;
}

impl Default for Breadcrumb {
	fn default() -> Self {
		Self::new()
	}
}

impl Breadcrumb {
	pub(crate) fn new() -> Self {
		glib::Object::new()
	}

	pub(crate) fn build(&self, path: &Path) {
		let imp = self.imp();

		imp.folder_icon.set_resource(Some("/com/bellbird/notes/icons/folder-closed.svg"));
		imp.note_icon.set_resource(Some("/com/bellbird/notes/icons/note.svg"));
		imp.directory_path.set_text(&self.get_prepared_path(path));
		imp.note_name.set_text(&self.get_note_name(path));
	}

	fn get_note_name(&self, path: &Path) -> String {
		let mut note = String::new();
		if path.is_file() {
			note = path
				.file_stem().unwrap()
				.to_str().unwrap()
				.to_string();
		}
		note
	}

	fn get_prepared_path(&self, path: &Path) -> String {
		let root_dir = Directories::root_directory()
			.display()
			.to_string();

		let mut directory = String::new();
		if path.is_file() {
			directory = path
				.parent().unwrap()
				.to_str().unwrap()
				.to_string();

			directory = directory.replace(&root_dir, "");
			let nbr_separators = directory.matches("/").count();
			directory = directory.replacen("/", "  ›  ", nbr_separators);
			directory.push_str("  › ");
		}
		directory
	}
}
