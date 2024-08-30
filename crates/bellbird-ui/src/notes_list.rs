use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use bellbird_core::notes::Notes;

use gtk::gio;
use gtk::prelude::*;

use crate::contextmenu::{BbMenuItem, BbMenuSection, ContextMenu};
use crate::notes_list_row::NotesListItem;

#[derive(Debug, Clone)]
pub struct NotesList {
	pub path: PathBuf,
	pub model: gio::ListStore,
	pub list_view: gtk::ListView,
	pub current_note: Rc<RefCell<PathBuf>>,
	pub selected_ctx_path: Rc<RefCell<PathBuf>>,
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

		let list_view = gtk::ListView::builder()
			.model(&selection_model)
			.factory(&factory)
			.vexpand(true)
			.valign(gtk::Align::Fill)
			.margin_top(5)
			.margin_bottom(5)
			//.margin_start(5)
			//.margin_end(5)
			.single_click_activate(true)
			.show_separators(true)
			.build();

		list_view.connect_activate(move |list_view, position| {
			let model = list_view.model().unwrap();
			let label = model.item(position).and_downcast::<gtk::Label>().unwrap();
			let path = label.widget_name();
			model.select_item(position, true);

			list_view
				.activate_action("app.open-note", Some(&path.to_variant()))
				.expect("The action `open-note` does not exist.");
		});

		Self {
			path: path.to_path_buf(),
			model,
			list_view,
			current_note: Rc::new(RefCell::new(path.to_path_buf())),
			selected_ctx_path: Rc::new(RefCell::new(path.to_path_buf())),
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

				self.model.append(&label)
			})
		}

		self.set_selection();
	}

	pub fn refresh(&mut self) {
		self.update_path(self.path.clone());
	}

	pub fn update_current_note(&self, path: PathBuf) {
		self.current_note.borrow_mut().set_file_name(path);
	}

	pub fn set_selected_ctx_note(&self, path: PathBuf) {
		self.selected_ctx_path.borrow_mut().set_file_name(path);
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

	fn build_context_menu(&self, app: &adw::Application) {
		let mut sections = vec![];
		let mut sec0 = vec![];
		sec0.push(BbMenuItem { label: "Open in New Tab", action: "open-note-in-tab" });
		sections.push(BbMenuSection { label: None, items: sec0 });

		let mut sec1 = vec![];
		sec1.push(BbMenuItem { label: "Create Note", action: "create-note" });
		sections.push(BbMenuSection { label: None, items: sec1 });

		let mut sec2 = vec![];
		sec2.push(BbMenuItem { label: "Duplicate Note", action: "duplicate-note" });
		sec2.push(BbMenuItem { label: "Pin / Unpin Note", action: "toggle-pin-note" });
		sec2.push(BbMenuItem { label: "Rename Note", action: "rename-note" });
		sections.push(BbMenuSection { label: None, items: sec2 });

		let mut sec3 = vec![];
		sec3.push(BbMenuItem { label: "Delete Note", action: "delete-note" });
		sections.push(BbMenuSection { label: None, items: sec3 });

		let app_clone = app.clone();
		let self_clone = self.clone();

		ContextMenu::new(sections, &self.list_view, 180).build(move |widget| {
			let actions = vec![
				"open-note-in-tab",
				"duplicate-note",
				"toggle-pin-note",
				"rename-note",
				"delete-note"
			];
			for action in actions.iter() {
				app_clone.action_enabled_changed(action, false);
			}

			if widget.widget_name() != "GtkListView" {
				match widget.parent() {
					Some(parent) => {
						#[allow(unused)]
						let mut should_activate_on_note_items = false;
						let mut note_path = PathBuf::from("");

						if parent.widget_name() == "NotesListRow" {
							should_activate_on_note_items = true;
							if let Some(label) = parent.last_child().and_downcast::<gtk::Label>() {
								note_path.push(label.label());
							}
						}
						else {
							should_activate_on_note_items = true;
							if let Some(first_child) = parent.first_child() {
								if let Some(label) = first_child.last_child().and_downcast::<gtk::Label>() {
									note_path.push(label.label());
								}
							}
						}

						self_clone.set_selected_ctx_note(note_path.clone().into());
						println!("{:?}", note_path);
						if should_activate_on_note_items {
							for action in actions.iter() {
								app_clone.action_enabled_changed(action, true);
							}
						}
					},
					None => println!("nope")
				}
			}
		});
	}

	fn _enable_actions(&self, app: &adw::Application, actions: Vec<&str>) {
		for action in actions.iter() {
			app.action_enabled_changed(action, true);
		}
	}

	fn _disable_actions(&self, app: adw::Application, actions: Vec<&str>) {
		for action in actions.iter() {
			app.action_enabled_changed(action, false);
		}
	}
}

pub fn build_ui(
	app: &adw::Application,
	notes_list: &Rc<RefCell<NotesList>>
) -> gtk::Box {
	let notes_panel = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.vexpand(true)
		.valign(gtk::Align::Fill)
		.width_request(195)
		.margin_top(3)
		.margin_bottom(3)
		.margin_end(2)
		.css_classes(["notes-panel"])
		.build();

	let notes_panel_label = gtk::Label::builder()
		.label("Notes")
		.margin_start(10)
		.margin_end(5)
		.margin_top(5)
		.margin_bottom(5)
		.halign(gtk::Align::Start)
		.build();

	let scrollable_window = gtk::ScrolledWindow::builder()
		.child(notes_list.borrow_mut().view())
		.hscrollbar_policy(gtk::PolicyType::External)
		.build();

	notes_list.borrow_mut().build_context_menu(app);

	notes_panel.append(&notes_panel_label);
	notes_panel.append(&scrollable_window);

	notes_panel
}
