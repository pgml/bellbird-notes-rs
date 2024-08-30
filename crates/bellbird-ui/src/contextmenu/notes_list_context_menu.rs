use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc};

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

	pub fn setup_context_menu_actions(self: Arc<Self>) {
		let app_clone = self.app.clone();

		let open_in_tab = gio::SimpleAction::new("open-note-in-tab", None);
		open_in_tab.connect_activate(move |_, _| println!("open in tab"));
		app_clone.add_action(&open_in_tab);

		// @todo try to make this whole thing less verbose
		let create_note = gio::SimpleAction::new("create-note", None);
		{
			let self_clone = Arc::clone(&self);
			create_note.connect_activate(move |_, _| self_clone.create_note());
		}
		app_clone.add_action(&create_note);

		let duplicate_note = gio::SimpleAction::new("duplicate-note", None);
		duplicate_note.connect_activate(move |_, _| println!("duplicate note"));
		app_clone.add_action(&duplicate_note);

		let pin_note = gio::SimpleAction::new("toggle-pin-note", None);
		pin_note.connect_activate(move |_, _| println!("pin note"));
		app_clone.add_action(&pin_note);

		let rename_note = gio::SimpleAction::new("rename-note", None);
		rename_note.connect_activate(move |_, _| println!("rename note"));
		rename_note.set_enabled(false);
		app_clone.add_action(&rename_note);

		let delete_note = gio::SimpleAction::new("delete-note", None);
		{
			let self_clone = Arc::clone(&self);
			delete_note.connect_activate(move |_, _| self_clone.delete_note());
			delete_note.set_enabled(false);
		}
		app_clone.add_action(&delete_note);
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
			"Enter note namee:",
			"New note",
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

	fn delete_note(&self) {
		let app_clone = self.app.clone();
		let notes_list_clone = self.notes_list.clone();
		let dialogue = Dialogue::new(&app_clone);
		// ...and this clone is somehow disgusting...try to make it less weird
		let notes_list_binding = notes_list_clone.clone();
		//dialogue.input(
		//	"Delete New Note",
		//	move |note| {
		//		let mut notes_list = notes_list_binding.borrow_mut();
		//		let mut path = PathBuf::from(&notes_list.path);
		//		path.push(&note);
		//		Notes::write_to_file(path, String::new());
		//		notes_list.refresh();
		//	},
		//	|| {}
		//)

	}
}
