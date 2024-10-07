use gtk::{
	glib,
	prelude::*,
	subclass::prelude::*,
};
use std::cell::RefCell;
use once_cell::sync::Lazy;

mod imp {
	use glib::property::PropertySet;

use super::*;

	#[derive(Debug)]
	pub(crate) struct ListModelItem {
		pub name: RefCell<String>,
		pub path: RefCell<String>,
		pub pinned: RefCell<bool>,
	}

	impl Default for ListModelItem {
		fn default() -> Self {
			Self {
				name: RefCell::new("".to_string()),
				path: RefCell::new("".to_string()),
				pinned: RefCell::new(false),
			}
		}
	}

	#[glib::object_subclass]
	impl ObjectSubclass for ListModelItem {
		const NAME: &'static str = "ListModelItem";
		type Type = super::ListModelItem;
		type ParentType = gtk::Widget;
	}

	impl ObjectImpl for ListModelItem {
		fn properties() -> &'static [glib::ParamSpec] {
			static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
				vec![
					glib::ParamSpecString::builder("name").default_value(None).build(),
					glib::ParamSpecString::builder("path").default_value(None).build(),
					glib::ParamSpecBoolean::builder("pinned").default_value(false).build(),
				]
			});
			PROPERTIES.as_ref()
		}

		fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
			match pspec.name() {
				"name" => self.name.borrow().to_value(),
				"path" => self.path.borrow().to_value(),
				"pinned" => self.pinned.borrow().to_value(),
				_ => unimplemented!(),
			}
		}

		fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
			match pspec.name() {
				"name" => {
					let name = value
						.get::<String>()
						.expect("The value needs to be of type `String`");
					self.name.set(name);
				},
				"path" => {
					let path = value
						.get::<String>()
						.expect("The value needs to be of type `String`");
					self.path.set(path);
				},
				"pinned" => {
					let pinned = value
						.get::<bool>()
						.expect("The value needs to be of type `bool`");
					self.pinned.set(pinned);
				}
				_ => unimplemented!(),
			}
		}
	}
	impl WidgetImpl for ListModelItem {}
}

glib::wrapper! {
	pub(crate) struct ListModelItem(ObjectSubclass<imp::ListModelItem>)
		@extends gtk::Widget;
}

impl Default for ListModelItem {
	fn default() -> Self {
		Self::new()
	}
}

impl ListModelItem {
	pub(crate) fn new() -> Self {
		glib::Object::new()
	}

	#[allow(unused)]
	pub(crate) fn name(&self) -> String {
		self.property::<String>("name")
	}

	#[allow(unused)]
	pub(crate) fn set_name(&self, name: &str) {
		self.set_property("name", name.to_value());
	}

	#[allow(unused)]
	pub(crate) fn path(&self) -> String {
		self.property::<String>("path")
	}

	#[allow(unused)]
	pub(crate) fn set_path(&self, path: &str) {
		self.set_property("path", path.to_value());
	}

	#[allow(unused)]
	pub(crate) fn is_pinned(&self) -> bool {
		self.property::<bool>("pinned")
	}

	#[allow(unused)]
	pub(crate) fn set_is_pinned(&self, pinned: bool) {
		self.set_property("pinned", pinned.to_value());
	}
}
