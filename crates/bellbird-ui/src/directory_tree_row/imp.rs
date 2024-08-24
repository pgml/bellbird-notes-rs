use gtk::{
	glib::{
		self,
	},
	subclass::prelude::*
};

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "../../data/ui/directory_tree_row.ui")]
pub struct DirectoryTreeRow {
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
impl ObjectSubclass for DirectoryTreeRow {
	const NAME: &'static str = "DirectoryTreeRow";
	type Type = super::DirectoryTreeRow;
	type ParentType = gtk::Box;

	fn class_init(klass: &mut Self::Class) {
		klass.bind_template();
	}

	fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
		obj.init_template();
	}
}

impl ObjectImpl for DirectoryTreeRow {
	// fn signals() -> &'static [Signal] {
	// 	static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
	// 	SIGNALS.get_or_init(|| {
	// 		vec![Signal::builder("activate")
	// 			.build()]
	// 	})
	//}

	//fn properties() -> &'static [glib::ParamSpec] {
	//	static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
	//		// we can use it to represent Option<String>
	//		vec![
	//			glib::ParamSpecString::builder("name")
	//				.default_value(None)
	//				.build(),
	//			glib::ParamSpecString::builder("path")
	//				.default_value(None)
	//				.build()
	//		]
	//	});
	//	PROPERTIES.as_ref()
	//}

	//fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
	//	match pspec.name() {
	//		"name" => self.name.to_value(),
	//		"path" => self.path.to_value(),
	//		_ => unimplemented!(),
	//	}
	//}

	//fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
	//	match pspec.name() {
	//		"name" => {
	//			let name = value
	//				.get::<Option<String>>()
	//				.expect("The value needs to be of type `Option<String>`");

	//			self.obj().set_name(name.clone());
	//			// self.name.replace(name);
	//		},
	//		"path" => {
	//			let path = value
	//				.get::<Option<String>>()
	//				.expect("The value needs to be of type `Option<String>`");

	//			self.obj().set_path(path.clone());
	//			// self.name.replace(name);
	//		}
	//		_ => unimplemented!(),
	//	}
	//}
}

impl BoxImpl for DirectoryTreeRow {}
impl WidgetImpl for DirectoryTreeRow {}
