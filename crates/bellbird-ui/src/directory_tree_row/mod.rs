pub(crate) mod imp;

use std::path::Path;

use gtk::{glib, pango::EllipsizeMode, prelude::*, subclass::prelude::*};

glib::wrapper! {
	pub struct DirectoryTreeRow(ObjectSubclass<imp::DirectoryTreeRow>)
		@extends gtk::Widget, gtk::Box;
}

impl Default for DirectoryTreeRow {
	fn default() -> Self {
		Self::new()
	}
}

impl DirectoryTreeRow {
	pub(crate) fn new() -> Self {
		glib::Object::new()
	}

	pub(crate) fn append_tree_item(
		&self,
		name: &str,
		path: &Path,
		depth_from_root: u32,
		has_children: bool
	) {
		let imp = self.imp();

		let indent_size = 15;
		let _expander = gtk::Image::builder()
			.resource("/com/bellbird/notes/icons/arrow-right.svg")
			.pixel_size(12)
			.build();
		//println!("{:?}", expander);
		//imp.expander.set_child(Some(&expander));

		let controller = gtk::GestureClick::new();
		controller.connect_released(move |gesture, n_press, x, y| {
			println!("{:?} {:?} {:?} {:?}", gesture, n_press, x, y);
		});
		imp.expander.add_controller(controller);
		imp.expander.set_hide_expander(!has_children);
		imp.expander.set_margin_start(depth_from_root as i32 * indent_size);

		imp.icon.set_resource(Some("/com/bellbird/notes/icons/folder-closed.svg"));

		//imp.name.set_text(&self.name());
		imp.name.set_text(name);
		imp.name.set_ellipsize(EllipsizeMode::End);
		//imp.path.set_text(&self.path());
		imp.path.set_text(&path.display().to_string());
	}
}
