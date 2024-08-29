use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::glib;

use bellbird_core::directories::Directories;
use bellbird_core::config::Config;
use bellbird_core::notes::Notes;

use crate::action_entries::ActionEntries;
use crate::contextmenu::notes_list_context_menu::NotesListContextMenu;
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

	app.connect_startup(|_| load_css());
	app.connect_activate(build_ui);

	app.run()
}

fn build_ui(app: &adw::Application) {
	let window_box = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.name("window-box")
		.build();

	let panels_wrapper = gtk::Box::builder()
		.orientation(gtk::Orientation::Horizontal)
		.name("panels-wrapper")
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
	let notes_list = Rc::new(RefCell::new(NotesList::new(&path)));
	let editor = Rc::new(RefCell::new(Editor::new(&note_path)));

	directory_tree.borrow_mut().update_current_directory(path.clone().into());
	directory_tree.borrow_mut().update_path(bellbird_root.to_path_buf());
	notes_list.borrow_mut().update_current_note(note_path.clone().into());
	notes_list.borrow_mut().update_path(path.to_path_buf());
	editor.borrow_mut().update_path(note_path.to_path_buf());

	on_startup(app, &notes_list);
	let action_entries = ActionEntries::new(
		&app,
		&editor,
		&notes_list,
		&directory_tree
	);
	action_entries.register_refresh_notes_action();
	action_entries.register_open_note_action();
	action_entries.register_editor_key_up();

	panels_wrapper.append(&directory_tree::build_ui(&directory_tree));
	panels_wrapper.append(&notes_list::build_ui(&notes_list));
	panels_wrapper.append(&editor_view::build_ui(&editor));

	window_box.append(&panels_wrapper);

	window.present();
}

fn load_css() {
	let provider = gtk::CssProvider::new();
	provider.load_from_string(default_layout::DEFAULT_STYLE);

	gtk::style_context_add_provider_for_display(
		&gtk::gdk::Display::default().expect("Could not connect to a display."),
		&provider,
		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);
}

fn on_startup(app: &adw::Application, notes_list: &Rc<RefCell<NotesList>>) {
	let notes_list_context_menu = NotesListContextMenu::new(app.clone(), notes_list.clone());
	notes_list_context_menu.setup_context_menu_actions();
}
