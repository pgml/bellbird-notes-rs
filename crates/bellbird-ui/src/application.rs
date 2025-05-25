use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use gtk::prelude::*;
use gtk::glib;

use bellbird_core::directories::Directories;
use bellbird_core::config::Config;
use bellbird_core::notes::Notes;

use crate::action_entries::ActionEntries;
use crate::contextmenu::directory_tree_context_menu::DirectoryTreeContextMenu;
use crate::contextmenu::notes_list_context_menu::NotesListContextMenu;
use crate::directory_tree::DirectoryTree;
use crate::editor_view::Editor;
use crate::notes_list::NotesList;
use crate::directory_tree;
use crate::editor_view;
use crate::default_layout;

pub fn run() -> glib::ExitCode {
	let config = Config::new();
	let app = adw::Application::builder()
		.application_id(config.app_id()).build();

	app.connect_startup(|_| load_css());
	//app.connect_activate(build_ui);
	app.connect_activate(|app| {
		build_ui(&app);
	});

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

	let bellbird_root = match Directories::bb_root_directory() {
		Some(bb_root) => bb_root,
		None => PathBuf::new()
	};

	let path = match Directories::current_directory_path() {
		Some(p) => p,
		None => PathBuf::new()
	};

	let note_path = match Notes::current_path() {
		Some(n) => n,
		None => PathBuf::new()
	};

	let directory_tree = Rc::new(RefCell::new(
		DirectoryTree::new(app, &bellbird_root)
	));
	let notes_list = Rc::new(RefCell::new(NotesList::new(&path)));
	let editor = Rc::new(RefCell::new(Editor::new(&note_path)));

	directory_tree.borrow_mut().update_current_directory(path.clone().into());
	directory_tree.borrow_mut().update_path(bellbird_root.to_path_buf());
	notes_list.borrow_mut().update_current_note(note_path.clone().into());

	glib::MainContext::default().spawn_local(glib::clone!(
		#[weak] notes_list, #[strong] note_path, #[weak] editor,
		async move {
			notes_list.borrow_mut().update_path(path.to_path_buf().into()).await;
			editor.borrow_mut().update_path(note_path.to_path_buf()).await;
		}
	));

	register_actions(&app, &directory_tree, &notes_list, &editor);

	panels_wrapper.append(&directory_tree::build_ui(&app, &directory_tree));
	panels_wrapper.append(&notes_list.borrow_mut().build_ui(&app));
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

fn register_actions(
	app: &adw::Application,
	directory_tree: &Rc<RefCell<DirectoryTree>>,
	notes_list: &Rc<RefCell<NotesList>>,
	editor: &Rc<RefCell<Editor>>,
) {
	Arc::new(NotesListContextMenu::new(app, notes_list.clone()))
		.setup_context_menu_actions();

	Arc::new(DirectoryTreeContextMenu::new(app, directory_tree.clone()))
		.setup_context_menu_actions();

	let action_entries = ActionEntries::new(
		&app,
		&directory_tree,
		&notes_list,
		&editor,
	);
	action_entries.register_refresh_notes_action();
	action_entries.register_open_note_action();
	action_entries.register_editor_key_up();
}
