pub(crate) mod imp;

//use std::path::Path;

use gtk::{glib, pango::EllipsizeMode, subclass::prelude::*};

use crate::directory_tree::TreeItem;

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
		item: &TreeItem,
		//dir_name: &str,
		//path: &Path,
		//depth_from_root: u32,
		//has_children: bool
	) {
		let imp = self.imp();
		let dir_name = item.name;
		let path = &item.path;
		//let path_clone = path.clone();

		//let indent_size = 15;
		//let _expander = gtk::Image::builder()
		//	.resource("/com/bellbird/notes/icons/arrow-right.svg")
		//	.pixel_size(12)
		//	.build();
		//imp.expander.set_child(Some(&_expander));

		//let gesture = gtk::GestureClick::new();
		//gesture.connect_released(move |gesture, n_press, x, y| {
		//	//println!("{:?} {:?} {:?} {:?}", gesture, n_press, x, y);
		//	println!("{:?}", path_clone);
		//});
		//imp.expander.add_controller(gesture);
		////println!("{:?}", item.path);

		//imp.expander.add_controller(controller);
		//let hide_children = if has_children { "show" } else { "hide" };
		//imp.expander.set_css_classes(&[hide_children]);
		////println!("{:?} {:?}", has_children, name);
		////println!("{:?} {:?}", depth_from_root as i32, indent_size);
		//imp.expander.set_margin_start(depth_from_root as i32 * indent_size);

		imp.icon.set_resource(Some("/com/bellbird/notes/icons/folder-closed.svg"));

		imp.name.set_text(dir_name);
		imp.name.set_ellipsize(EllipsizeMode::End);
		imp.path.set_text(&path.display().to_string());
	}
}
