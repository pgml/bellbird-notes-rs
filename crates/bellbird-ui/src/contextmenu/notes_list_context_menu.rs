use std::{cell::RefCell, path::PathBuf, rc::Rc};

use bellbird_core::notes::Notes;
use gtk::{gio, prelude::*};

use crate::{dialogue::Dialogue, notes_list::NotesList};

#[derive(Debug, Clone)]
pub struct NotesListContextMenu {
	app: adw::Application,
	notes_list: Rc<RefCell<NotesList>>,
}

impl NotesListContextMenu {
	pub fn new(
		app: adw::Application,
		notes_list: Rc<RefCell<NotesList>>
	) -> Self {
		Self {
			app,
			notes_list
		}
	}

	pub fn setup_context_menu_actions(self) {
		let app_clone = self.app.clone();
		let create_note = gio::ActionEntry::builder("create-note")
			.activate(move |_, _, _| self.create_note())
			.build();

		let rename_note = gio::ActionEntry::builder("rename-note")
			.activate(|_, _, _| println!("rename note was pressed"))
			.build();

		let delete_note = gio::ActionEntry::builder("delete-note")
			.activate(|_, _, _| println!("delete note was pressed"))
			.build();

		app_clone.add_action_entries([create_note, rename_note, delete_note]);
	}

	fn create_note(&self) {
		// this clone...
		let app_clone = self.app.clone();
		let notes_list_clone = self.notes_list.clone();
		let dialogue = Dialogue::new(&app_clone);
		// ...and this clone is somehow disgusting...try to make it less weird
		let notes_list_binding = notes_list_clone.clone();
		dialogue.input(
			"Create New Note",
			move |note| {
				let mut notes_list = notes_list_binding.borrow_mut();
				let mut path = PathBuf::from(&notes_list.path);
				path.push(&note);
				Notes::write_to_file(path, String::new());
				notes_list.refresh();
			},
			|| {}
		)
	}
}
