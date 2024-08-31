use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;

use gtk::gio;

use crate::editor_view::Editor;
use crate::{
	directory_tree::DirectoryTree,
	notes_list::NotesList
};

pub struct ActionEntries<'a> {
	pub app: &'a adw::Application,
	pub editor: &'a Rc<RefCell<Editor>>,
	pub notes_list: &'a Rc<RefCell<NotesList>>,
	pub directory_tree: &'a Rc<RefCell<DirectoryTree>>,
}

impl<'a> ActionEntries<'a> {
	pub fn new(
		app: &'a adw::Application,
		directory_tree: &'a Rc<RefCell<DirectoryTree>>,
		notes_list: &'a Rc<RefCell<NotesList>>,
		editor: &'a Rc<RefCell<Editor>>,
	) -> Self {
		Self {
			app,
			editor,
			notes_list,
			directory_tree
		}
	}

	pub fn register_refresh_notes_action(&self) {
		let notes_list_clone = self.notes_list.clone();
		let directory_tree_clone = self.directory_tree.clone();
		let action_refresh_notes = gio::ActionEntry::builder("refresh-notes")
			.parameter_type(Some(&String::static_variant_type()))
			.activate(move |_, _action, parameter| {
				let path = parameter
					.expect("Could not get Parameter")
					.get::<String>()
					.expect("The variant nees to be of type `String`");
				let path_buf = std::path::PathBuf::from(path.clone());

				directory_tree_clone.borrow_mut().update_current_directory(path.clone().into());
				notes_list_clone.borrow_mut().update_path(path_buf);
			})
			.build();

		self.app.add_action_entries([action_refresh_notes]);
	}

	pub fn register_open_note_action(&self) {
		let editor_clone = self.editor.clone();
		let notes_list_clone = self.notes_list.clone();
		let action_open_notes = gio::ActionEntry::builder("open-note")
			.parameter_type(Some(&String::static_variant_type()))
			.activate(move |_, _action, parameter| {
				let path = parameter
					.expect("Could not get Parameter")
					.get::<String>()
					.expect("The variant needs to be of type `String`");
				notes_list_clone.borrow_mut().update_current_note(path.clone().into());
				editor_clone.borrow_mut().update_path(path.into());
			})
			.build();

		self.app.add_action_entries([action_open_notes]);
	}

	pub fn register_editor_key_up(&self) {
		let editor_clone = self.editor.clone();
		let action_editor_key_up = gio::ActionEntry::builder("editor-key-up")
			.parameter_type(Some(&String::static_variant_type()))
			.activate(move |_, _, _| {
				editor_clone.borrow_mut().write_note();
			})
			.build();

		self.app.add_action_entries([action_editor_key_up]);
	}

	pub fn register_context_create_note(&self) {
		let action_create_note = gio::ActionEntry::builder("create-note")
			.parameter_type(Some(&String::static_variant_type()))
			.activate(move |_, _, _| {
				println!("asdasd");
			})
			.build();
		self.app.add_action_entries([action_create_note]);
	}
}
