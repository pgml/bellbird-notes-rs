use std::sync::OnceLock;

use gtk::{
	glib::{
		self,
		subclass::Signal,
	},
	prelude::ObjectExt,
	subclass::prelude::*
};

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "../../data/ui/list_view_row.ui")]
pub struct ListViewItem {
	#[template_child]
	pub name: TemplateChild<gtk::Label>,
	#[template_child]
	pub path: TemplateChild<gtk::Label>,
	// #[template_child]
	// pub description: TemplateChild<gtk::Label>,
	// #[template_child]
	// pub image: TemplateChild<gtk::Image>,
}

#[glib::object_subclass]
impl ObjectSubclass for ListViewItem {
	const NAME: &'static str = "ListViewRow";
	type Type = super::ListViewItem;
	type ParentType = gtk::Box;

	fn class_init(klass: &mut Self::Class) {
		klass.bind_template();
	}

	fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
		obj.init_template();
	}
}

impl ObjectImpl for ListViewItem {
	fn signals() -> &'static [Signal] {
		static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
		SIGNALS.get_or_init(|| {
			vec![Signal::builder("activate")
				.build()]
		})
	}

	// fn properties() -> &'static [glib::ParamSpec] {
	// 	static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
	// 	PROPERTIES.get_or_init(|| {
	// 		vec![
	// 			glib::ParamSpecString::builder("name").default_value("Name").build(),
	// 			glib::ParamSpecString::builder("path").default_value("Path").build(),
	// 		]
	// 	})
	// }

	// fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
	// 	// let obj = self.obj();

	// 	match pspec.name() {
	// 		"name" => {
	// 			let name: Option<String> = value.get().expect("Type mismatch");
	// 			if let Some(name) = name {
	// 				self.name.set_text(&name);
	// 			}
	// 		},
	// 		"path" => {
	// 			let path: Option<String> = value.get().expect("Type mismatch");
	// 			if let Some(path) = path {
	// 				self.path.set_text(&path);
	// 			}
	// 		}
	// 		_ => unimplemented!(),
	// 	}
	// }

	// fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
	// 	match pspec.name() {
	// 		"name" => self.name.text().to_value(),
	// 		"path" => self.path.text().to_value(),
	// 		_ => unimplemented!(),
	// 	}
	// }
}

impl BoxImpl for ListViewItem {}
impl WidgetImpl for ListViewItem {}

impl ListViewItem {
	pub fn emit_activate(&self) {
		self.obj().emit_by_name::<()>("activated", &[]);
	}
}
