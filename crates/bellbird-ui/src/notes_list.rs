use std::{cell::RefCell, path::{Path, PathBuf}, rc::Rc};

use bellbird_core::notes::Notes;

use gtk::{
	gio,
	prelude::*
};

use crate::notes_list_row::NotesListItem;

#[derive(Debug, Clone)]
pub struct NotesList {
	pub path: PathBuf,
	pub model: gio::ListStore,
	pub list_view: gtk::ListView,
}

impl<'a> NotesList {
	pub fn new(path: &'a Path) -> Self {
		let model = gio::ListStore::new::<gtk::Label>();
		let model_clone = model.clone();

		let factory = gtk::SignalListItemFactory::new();
		factory.connect_setup(move |_factory, item| {
			let item = item.downcast_ref::<gtk::ListItem>().unwrap();
			let row = NotesListItem::default();
			item.set_child(Some(&row));
		});

		factory.connect_bind(move |_factory, item| {
			let item = item.downcast_ref::<gtk::ListItem>().unwrap();
			let label = item.item().and_downcast::<gtk::Label>().unwrap();
			let child = item.child().and_downcast::<NotesListItem>().unwrap();
			child.append_tree_item(&label);
		});

		// A sorter used to sort AppInfo in the model by their name
		let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
			let app_info1 = obj1.downcast_ref::<gtk::Label>().unwrap();
			let _app_info2 = obj2.downcast_ref::<gtk::Label>().unwrap();

			app_info1
				.label()
				.to_lowercase()
				.cmp(&app_info1.label().to_lowercase())
				.into()
		});
		let sorted_model = gtk::SortListModel::new(Some(model_clone), Some(sorter));
		let selection_model = gtk::SingleSelection::new(Some(sorted_model));
		selection_model.set_autoselect(false);

		//let list_view = gtk::ListView::builder()
		let list_view = gtk::ListView::builder()
			.model(&selection_model)
			.factory(&factory)
			.vexpand(true)
			.valign(gtk::Align::Fill)
			.margin_top(5)
			.margin_end(5)
			.margin_bottom(5)
			.margin_start(5)
			.single_click_activate(true)
			.show_separators(true)
			.build();

		list_view.connect_activate(move |list_view, position| {
			let model = list_view.model().unwrap();
			let tree_item = model.item(position).and_downcast::<gtk::Label>().unwrap();
			let path = tree_item.widget_name();
			list_view
				.activate_action("win.open-note", Some(&path.to_variant()))
				.expect("The action `open-note` does not exist.");
		});

		Self {
			path: path.to_path_buf(),
			model,
			list_view
		}
	}

	pub fn update_path(&mut self, path: PathBuf) {
		self.path = path.clone();
		self.model.remove_all();

		if let Ok(notes) =  Notes::list(&path) {
			notes.iter().for_each(|note| {
				let label = gtk::Label::builder()
					.label(&note.name)
					.name(&note.path)
					.build();

				// println!("{:?}", label);
				self.model.append(&label)
			})
		}
	}

	pub fn view(&self) -> &gtk::ListView {
		&self.list_view
	}
}

pub fn build_ui(notes_list: Rc<RefCell<NotesList>>) -> gtk::Box {
	// notes_list.update_path(path);

	let notes_panel = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.vexpand(true)
		.valign(gtk::Align::Fill)
		.width_request(200)
		.margin_top(5)
		.margin_top(5)
		.margin_bottom(5)
		.margin_bottom(5)
		.css_classes(["notes-panel"])
		.build();

	let notes_panel_label = gtk::Label::builder()
		.label("Notes")
		.margin_top(5)
		.margin_start(5)
		.margin_bottom(5)
		.halign(gtk::Align::Start)
		.build();

	// let notes_list = NotesList::new("");
	let scrollable_window = gtk::ScrolledWindow::builder()
		.child(notes_list.borrow_mut().view())
		.build();

	notes_panel.append(&notes_panel_label);
	notes_panel.append(&scrollable_window);

	notes_panel
}
