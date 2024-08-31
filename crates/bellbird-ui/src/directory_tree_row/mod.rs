pub(crate) mod imp;

//use std::path::Path;

use gtk::{glib, pango::EllipsizeMode, subclass::prelude::*};
//use adw::prelude::*;

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
		_app: &adw::Application,
		item: &TreeItem,
		//dir_name: &str,
		//path: &Path,
		_depth_from_root: u32,
		_has_children: bool
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

		////println!("{:?}", app.);
		//let gesture = gtk::GestureClick::new();
		//gesture.connect_released(move |gesture, n_press, x, y| {
		//	//println!("{:?} {:?} {:?} {:?}", gesture, n_press, x, y);
		//	println!("{:?} expander", path_clone);
		//});
		//imp.expander.add_controller(gesture);
		//let hide_children = if has_children { "show" } else { "hide" };
		//imp.expander.set_css_classes(&[hide_children]);
		//imp.expander.set_margin_start(depth_from_root as i32 * indent_size);

		//let gesture = gtk::GestureClick::new();
		//let path_clone = path.clone();
		//let tree_row_content = imp.tree_row_content.clone();
		//let app_clone = app.clone();
		//gesture.connect_released(move |gesture, n_press, x, y| {
		//	//println!("{:?} {:?} {:?} {:?}", gesture, n_press, x, y);

		//	app_clone
		//		.activate_action("app.refresh-notes", Some(&path_clone.to_variant()));
		//	println!("{:?} row-content", path_clone);
		//});
		//imp.tree_row_content.add_controller(gesture);

		imp.icon.set_resource(Some("/com/bellbird/notes/icons/folder-closed.svg"));

		imp.name.set_text(dir_name);
		imp.name.set_ellipsize(EllipsizeMode::End);
		imp.path.set_text(&path.display().to_string());
	}
}
