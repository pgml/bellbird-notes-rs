use gtk::subclass::prelude::*;
use gtk::glib;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "../../data/ui/directory_tree_row.ui")]
pub struct DirectoryTreeRow {
	#[template_child]
	pub expander: TemplateChild<gtk::TreeExpander>,
	#[template_child]
	pub icon: TemplateChild<gtk::Image>,
	#[template_child]
	pub name: TemplateChild<gtk::Label>,
	#[template_child]
	pub path: TemplateChild<gtk::Label>,
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

impl ObjectImpl for DirectoryTreeRow { }
impl BoxImpl for DirectoryTreeRow {}
impl WidgetImpl for DirectoryTreeRow {}
