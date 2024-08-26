mod imp;

use gtk::{glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
	pub struct NotesListItem(ObjectSubclass<imp::NotesListItem>)
		@extends gtk::Widget, gtk::Box;
}

impl Default for NotesListItem {
	fn default() -> Self {
		glib::Object::new()
	}
}

impl NotesListItem {
	pub fn new()  {
		// let object = Object::new();
	}

	pub fn append_tree_item(&self, item_label: &gtk::Label) {
		let imp = self.imp();

		imp.icon.set_resource(Some("/com/bellbird/notes/icons/note.svg"));
		imp.name.set_text(&item_label.label());
		imp.path.set_text(&item_label.widget_name())
	}
}
