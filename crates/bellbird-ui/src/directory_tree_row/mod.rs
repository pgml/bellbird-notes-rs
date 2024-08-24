mod imp;

use gtk::{glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
	pub struct DirectoryTreeRow(ObjectSubclass<imp::DirectoryTreeRow>)
		@extends gtk::Widget, gtk::Box;
}

// impl Default for DirectoryTreeRow {
// 	fn default() -> Self {
// 		glib::Object::new()
// 	}
// }

impl DirectoryTreeRow {
	pub fn new() -> Self {
		glib::Object::new()
	}

	pub fn append_tree_item(&self, item_label: &gtk::Label) {
		let imp = self.imp();
		// println!("{:?} {:?}, asdas", &item_label.text(), &item_label.text());
		imp.name.set_text(&item_label.label());
		imp.path.set_text(&item_label.widget_name())

		// if let Some(desc) = app_info.description() {
		// 	imp.description.set_text(&desc);
		// }

		// if let Some(icon) = app_info.icon() {
		// 	imp.image.set_from_gicon(&icon);
		// }
	}

	//#[allow(unused)]
	//pub(crate) fn name(&self) -> Option<String> {
	//	self.property::<Option<String>>("name")
	//}

	//#[allow(unused)]
	//pub(crate) fn set_name(&self, name: Option<String>) {
	//	self.set_property("name", name.to_value());
	//}

	//#[allow(unused)]
	//pub(crate) fn path(&self) -> Option<String> {
	//	self.property::<Option<String>>("path")
	//}

	//#[allow(unused)]
	//pub(crate) fn set_path(&self, path: Option<String>) {
	//	self.set_property("path", path.to_value());
	//}
}
