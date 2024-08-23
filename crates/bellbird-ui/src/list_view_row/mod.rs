mod imp;

use gtk::{glib::{self}, prelude::*, subclass::prelude::*};

glib::wrapper! {
	pub struct ListViewItem(ObjectSubclass<imp::ListViewItem>)
		@extends gtk::Widget, gtk::Box;
}

impl Default for ListViewItem {
	fn default() -> Self {
		glib::Object::new()
	}
}

impl ListViewItem {
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
}
