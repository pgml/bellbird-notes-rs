//use std::cell::RefCell;
//use std::path::PathBuf;
//use std::rc::Rc;

use gtk::subclass::prelude::*;
use gtk::glib;

//use crate::notes_list::NotesList;

#[derive(Debug, gtk::CompositeTemplate)]
#[template(resource = "/com/bellbird/notes/ui/directory_tree_row.ui")]
pub struct DirectoryTreeRow {
	#[template_child]
	pub tree_row_wrapper: TemplateChild<gtk::Box>,
	#[template_child]
	pub tree_row_content: TemplateChild<gtk::Box>,
	#[template_child]
	pub expander: TemplateChild<gtk::TreeExpander>,
	#[template_child]
	pub icon: TemplateChild<gtk::Image>,
	#[template_child]
	pub name: TemplateChild<gtk::Label>,
	#[template_child]
	pub path: TemplateChild<gtk::Label>,
}

impl Default for DirectoryTreeRow {
	fn default() -> Self {
		Self {
			tree_row_wrapper: TemplateChild::<gtk::Box>::default(),
			tree_row_content: TemplateChild::<gtk::Box>::default(),
			expander: TemplateChild::<gtk::TreeExpander>::default(),
			icon: TemplateChild::<gtk::Image>::default(),
			name: TemplateChild::<gtk::Label>::default(),
			path: TemplateChild::<gtk::Label>::default(),
		}
	}
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

impl ObjectImpl for DirectoryTreeRow {}
impl BoxImpl for DirectoryTreeRow {}
impl WidgetImpl for DirectoryTreeRow {}
