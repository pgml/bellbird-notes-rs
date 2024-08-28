use gtk::subclass::prelude::*;
use gtk::glib;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/bellbird/notes/ui/breadcrumb.ui")]
pub struct Breadcrumb {
	#[template_child]
	pub folder_icon: TemplateChild<gtk::Image>,
	#[template_child]
	pub directory_path: TemplateChild<gtk::Label>,
	#[template_child]
	pub note_icon: TemplateChild<gtk::Image>,
	#[template_child]
	pub note_name: TemplateChild<gtk::Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for Breadcrumb {
	const NAME: &'static str = "Breadcrumb";
	type Type = super::Breadcrumb;
	type ParentType = gtk::Box;

	fn class_init(klass: &mut Self::Class) {
		klass.bind_template();
	}

	fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
		obj.init_template();
	}
}

impl ObjectImpl for Breadcrumb { }
impl BoxImpl for Breadcrumb {}
impl WidgetImpl for Breadcrumb {}
