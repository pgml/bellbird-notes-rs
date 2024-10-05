mod imp;

use std::path::{Path, PathBuf};
use gtk::{glib, subclass::prelude::*};
//use gtk::prelude::*;

glib::wrapper! {
	pub struct NotesListItem(ObjectSubclass<imp::NotesListItem>)
		@extends gtk::Widget, gtk::Box;
}

impl Default for NotesListItem {
	fn default() -> Self {
		NotesListItem::new()
	}
}

impl NotesListItem {
  pub const NONE: Option<&'static NotesListItem> = None;

	pub fn new() -> Self {
		glib::Object::new()
	}

	pub fn append_tree_item(
		&self,
		name: &str,
		path: PathBuf,
		_pinned: bool,
	) {
		let imp = self.imp();

		imp.icon.set_resource(Some("/com/bellbird/notes/icons/note.svg"));

		imp.name.set_text(&name);
		imp.path.set_text(&path.display().to_string());
	}

	pub fn set_icon(&self) {
		self.imp().icon.set_resource(Some("/com/bellbird/notes/icons/note.svg"));
	}

	pub fn set_name(&self, name: &str) {
		self.imp().name.set_text(name);
	}

	pub fn set_path(&self, path: &Path) {
		self.imp().path.set_text(&path.display().to_string());
	}
}
