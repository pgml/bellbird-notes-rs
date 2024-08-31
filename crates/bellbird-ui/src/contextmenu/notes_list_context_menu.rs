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
		app: &adw::Application,
		notes_list: Rc<RefCell<NotesList>>
	) -> Self {
		Self {
			app: app.clone(),
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
		{
			let self_clone = Arc::clone(&self);
			rename_note.connect_activate(move |_, _| self_clone.rename_note());
		}
		app_clone.add_action(&rename_note);

		let delete_note = gio::SimpleAction::new("delete-note", None);
		{
			let self_clone = Arc::clone(&self);
			delete_note.connect_activate(move |_, _| self_clone.delete_note());
		}
		app_clone.add_action(&delete_note);
	}

	fn create_note(&self) {
		let notes_list_clone = self.notes_list.clone();
		let dialogue = Dialogue::new(&self.app);
		dialogue.input(
			"Create New Note",
			"Enter note namee:",
			"New note",
			move |note| {
				let mut path = PathBuf::from(notes_list_clone.borrow_mut()
					                           .path.to_str().unwrap_or(""));
				path.push(&note);
				Notes::write_to_file(path, String::new());
				notes_list_clone.borrow_mut().refresh();
			},
			|| {}
		)
	}

	fn rename_note(&self) {
		let notes_list = self.notes_list.clone();
		let dialogue = Dialogue::new(&self.app);
		let pathbuf_rc = self.notes_list.borrow_mut().selected_ctx_path.clone();
		let (note_path, file_stem) = self.get_path_and_stem(&pathbuf_rc);
		dialogue.input(
			"Rename Note",
			&format!("Rename ´{}´ to:", file_stem),
			&file_stem,
			move |note| {
				let mut new_path = PathBuf::from(notes_list.borrow_mut()
					                           .path.to_str().unwrap_or(""));
				new_path.push(&note);
				let old_path = PathBuf::from(&note_path);
				Notes::rename(old_path, new_path);
				notes_list.borrow_mut().refresh();
			},
			|| {}
		)
	}

	fn delete_note(&self) {
		// this whole thing is pretty ugly
		// but works for now
		let app_clone = self.app.clone();
		let notes_list_clone = self.notes_list.clone();
		let dialogue = Dialogue::new(&app_clone);
		let pathbuf_rc = self.notes_list.borrow_mut().selected_ctx_path.clone();
		let (note_path, file_stem) = self.get_path_and_stem(&pathbuf_rc);
		dialogue.warning_yes_no(
			"Delete New Note",
			"Do you really want to delete this note?",
			&format!("´{}´", file_stem),
			move || {
				Notes::delete(&PathBuf::from(&note_path));
				notes_list_clone.borrow_mut().refresh();
			},
			|| {}
		)
	}

	fn get_path_and_stem(&self, path: &Rc<RefCell<PathBuf>>) -> (String, String) {
		let note_path = path.borrow_mut();
		let file_stem = note_path.file_stem().unwrap()
			              .to_str().unwrap().to_string();
		let note_path = note_path.display().to_string();
		(note_path, file_stem)
	}
}
