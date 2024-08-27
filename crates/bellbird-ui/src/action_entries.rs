use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;

use gtk::gio;

use crate::{
	directory_tree::DirectoryTree,
	editor_view::Editor,
	notes_list::NotesList
};

pub fn register_update_notes_action(
	window: &gtk::ApplicationWindow,
	notes_list: &Rc<RefCell<NotesList>>,
	directory_tree: &Rc<RefCell<DirectoryTree>>
) {
	let notes_list_clone = notes_list.clone();
	let directory_tree_clone = directory_tree.clone();
	let action_update_notes = gio::ActionEntry::builder("update-notes")
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

	window.add_action_entries([action_update_notes]);
}

pub fn register_open_note_action(
	window: &gtk::ApplicationWindow,
	editor: &Rc<RefCell<Editor>>,
	notes_list: &Rc<RefCell<NotesList>>
) {
	let editor_clone = editor.clone();
	let notes_list_clone = notes_list.clone();
	let action_open_notes = gio::ActionEntry::builder("open-note")
		.parameter_type(Some(&String::static_variant_type()))
		.activate(move |_, _action, parameter| {
			let path = parameter
				.expect("Could not get Parameter")
				.get::<String>()
				.expect("The variant nees to be of type `String`");
			notes_list_clone.borrow_mut().update_current_note(path.clone().into());
			editor_clone.borrow_mut().update_path(path.into());
		})
		.build();

	window.add_action_entries([action_open_notes]);
}

pub fn register_editor_key_up(
	window: &gtk::ApplicationWindow,
	editor: &Rc<RefCell<Editor>>
) {
	let editor_clone = editor.clone();
	let action_editor_key_up = gio::ActionEntry::builder("editor-key-up")
		.parameter_type(Some(&String::static_variant_type()))
		.activate(move |_, _, _| {
			editor_clone.borrow_mut().write_note();
		})
		.build();

	window.add_action_entries([action_editor_key_up]);
}
