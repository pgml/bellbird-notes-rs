use std::cell::RefCell;
use std::rc::Rc;

use gtk::gio::ActionEntry;
use gtk::prelude::*;
use gtk::{glib::{self}, ApplicationWindow};

use bellbird_core::directories::Directories;
use bellbird_core::config::Config;
use bellbird_core::notes::Notes;

use crate::directory_tree::DirectoryTree;
use crate::editor_view::Editor;
use crate::notes_list::NotesList;
use crate::directory_tree;
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
	let window = ApplicationWindow::builder()
		.application(app)
		//.titlebar(&gtk::Box::new(gtk::Orientation::Horizontal, 0))
		.title(config.app_name())
		.default_width(1000)
		.default_height(600)
		.child(&window_box)
		.build();

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
	//editor.borrow_mut().update_current_note(path.clone().into());
	editor.borrow_mut().update_path(note_path.to_path_buf());

	register_update_notes_action(&window, &notes_list, &directory_tree);
	register_open_note_action(&window, &editor, &notes_list);

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

fn register_update_notes_action(
	window: &ApplicationWindow,
	notes_list: &Rc<RefCell<NotesList>>,
	directory_tree: &Rc<RefCell<DirectoryTree>>
) {
	let notes_list_clone = notes_list.clone();
	let directory_tree_clone = directory_tree.clone();
	let action_update_notes = ActionEntry::builder("update-notes")
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

fn register_open_note_action(
	window: &ApplicationWindow,
	editor: &Rc<RefCell<Editor>>,
	notes_list: &Rc<RefCell<NotesList>>
) {
	let editor_clone = editor.clone();
	let notes_list_clone = notes_list.clone();
	let action_open_notes = ActionEntry::builder("open-note")
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
