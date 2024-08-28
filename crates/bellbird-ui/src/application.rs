use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::glib;

use bellbird_core::directories::Directories;
use bellbird_core::config::Config;
use bellbird_core::notes::Notes;

use crate::action_entries::ActionEntries;
use crate::directory_tree::DirectoryTree;
use crate::editor_view::Editor;
use crate::notes_list::NotesList;
use crate::{action_entries, directory_tree};
use crate::notes_list;
use crate::editor_view;
use crate::default_layout;

pub fn run() -> glib::ExitCode {
	let config = Config::new();
	let app = adw::Application::builder().application_id(config.app_id()).build();

	// Connect to "activate" signal of `app`
	app.connect_startup(|_| load_css());
	app.connect_activate(build_ui);
	// Run the application
	app.run()
}

fn build_ui(app: &adw::Application) {
	let window_box = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.build();

	let panels_wrapper = gtk::Box::builder()
		.orientation(gtk::Orientation::Horizontal)
		.spacing(0)
		.build();

	let config = Config::new();
	let window = gtk::ApplicationWindow::new(app);

	//window.set_titlebar(Some(&gtk::Box::new(gtk::Orientation::Horizontal, 0)));
	window.set_title(Some(&config.app_name()));
	window.set_default_size(1000, 600);
	window.set_child(Some(&window_box));

	let bellbird_root = Directories::root_directory();
	let path = Directories::current_directory_path();
	let note_path = Notes::current_note_path();

	let directory_tree = Rc::new(RefCell::new(DirectoryTree::new(&bellbird_root)));
	directory_tree.borrow_mut().update_current_directory(path.clone().into());
	directory_tree.borrow_mut().update_path(bellbird_root.to_path_buf());

	let notes_list = Rc::new(RefCell::new(NotesList::new(&path)));
	notes_list.borrow_mut().update_current_note(note_path.clone().into());
	notes_list.borrow_mut().update_path(path.to_path_buf());

	let editor = Rc::new(RefCell::new(Editor::new(&note_path)));
	editor.borrow_mut().update_path(note_path.to_path_buf());

	let action_entries = ActionEntries::new(
		&window,
		&app,
		&editor,
		&notes_list,
		&directory_tree
	);
	action_entries.register_update_notes_action();
	action_entries.register_open_note_action();
	action_entries.register_editor_key_up();

	panels_wrapper.append(&directory_tree::build_ui(directory_tree));
	panels_wrapper.append(&notes_list::build_ui(notes_list));
	panels_wrapper.append(&editor_view::build_ui(editor));

	window_box.append(&panels_wrapper);

	// Present window
	window.present();
}

fn load_css() {
	// Load the CSS file and add it to the provider
	let provider = gtk::CssProvider::new();
	provider.load_from_string(default_layout::DEFAULT_STYLE);

	// Add the provider to the default screen
	gtk::style_context_add_provider_for_display(
		&gtk::gdk::Display::default().expect("Could not connect to a display."),
		&provider,
		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);
}
