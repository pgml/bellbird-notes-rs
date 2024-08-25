use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use bellbird_core::notes::Notes;

use gtk::{
	gio, prelude::*
};

use crate::notes_list_row::NotesListItem;

#[derive(Debug, Clone)]
pub struct NotesList {
	pub path: PathBuf,
	pub model: gio::ListStore,
	pub list_view: gtk::ListView,
	pub current_note: Rc<RefCell<PathBuf>>,
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
			item.set_selectable(false);
			child.append_tree_item(&label);
		});

		let selection_model = gtk::MultiSelection::new(Some(model_clone));

		//let list_view = gtk::ListView::builder()
		let list_view = gtk::ListView::builder()
			.model(&selection_model)
			.factory(&factory)
			.vexpand(true)
			.valign(gtk::Align::Fill)
			.margin_top(5)
			.margin_bottom(5)
			.margin_start(5)
			.margin_end(5)
			.single_click_activate(true)
			.show_separators(true)
			.build();

		let current_note = Rc::new(RefCell::new(path.to_path_buf()));
		//let current_note_clone = current_note.clone();
		list_view.connect_activate(move |list_view, position| {
			let model = list_view.model().unwrap();
			let label = model.item(position).and_downcast::<gtk::Label>().unwrap();
			let path = label.widget_name();
			model.select_item(position, true);

			Notes::set_current_note_path(&PathBuf::from(path.clone()));

			list_view
				.activate_action("win.open-note", Some(&path.to_variant()))
				.expect("The action `open-note` does not exist.");
		});

		Self {
			path: path.to_path_buf(),
			model,
			list_view,
			current_note,
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

		self.set_selection();
	}

	pub fn update_current_note(&self, path: PathBuf) {
		self.current_note.borrow_mut().set_file_name(path);
	}

	fn view(&self) -> &gtk::ListView {
		&self.list_view
	}

	fn set_selection(&self) {
		let current_note = self.current_note.clone();
		if let Some(selection_model) = self.list_view.model() {
			for index in 0..selection_model.n_items() {
				if let Some(item) = selection_model.item(index) {
					let path = item.downcast::<gtk::Label>().unwrap().widget_name();
					if path.to_string() == current_note.borrow_mut().display().to_string() {
						selection_model.select_item(index, true);
						break;
					}
				}
			}
		}
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
