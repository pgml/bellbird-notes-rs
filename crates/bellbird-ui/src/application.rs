use std::cell::RefCell;
use std::rc::Rc;

use gtk::gio::ActionEntry;
use gtk::prelude::*;
use gtk::{glib, ApplicationWindow};

use crate::editor_view::Editor;
use crate::notes_list::NotesList;
use crate::{
	default_layout,
	directory_tree,
	editor_view,
	notes_list
};

#[derive(Debug, Clone)]
pub struct App {
	pub id: String,
	pub title: String,
}

impl App {
	pub fn new() -> Self {
		let mut id = String::new();
		let mut title = String::new();

		if cfg!(feature = "stable") {
			id = String::from("org.bellbird.notes");
			title = String::from("Bellbird Notes");
		}

		if cfg!(feature = "snapshot") {
			id = String::from("org.bellbird.notes-snapshot");
			title = String::from("Bellbird Notes Snapshot");
		}

		println!("{id}, {title}");
		Self {
			id,
			title
		}
	}
}


pub fn run() -> glib::ExitCode {
	// Create a new application
	let bellbird = App::new();
	let app = adw::Application::builder().application_id(bellbird.id).build();
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
		.spacing(5)
		.build();

	//let headerbar = gtk::HeaderBar::new();
	let bellbird = App::new();
	let window = ApplicationWindow::builder()
		.application(app)
		.title(bellbird.title)
		.default_width(1000)
		.default_height(600)
		//.titlebar(&headerbar)
		.child(&window_box)
		.build();

	//let path = "/home/rico/.bellbird-notes/Bands/Stay Puft/Texte/";
	//let note_path = "/home2/pgml/Projekte/Godot/dear-guests/Characters/Scripts/Controller.cs";
	let path = "";
	let note_path = "";
	let notes_list = Rc::new(RefCell::new(NotesList::new(path)));
	notes_list.borrow_mut().update_path(path);

	let editor = Rc::new(RefCell::new(Editor::new(note_path)));
	editor.borrow_mut().update_path(note_path);

	register_update_notes_action(&window, &notes_list);
	register_open_note_action(&window, &editor);

	panels_wrapper.append(&directory_tree::build_ui());
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

fn register_update_notes_action(window: &ApplicationWindow, notes_list: &Rc<RefCell<NotesList>>) {
	let notes_list_clone = notes_list.clone();
	let action_update_notes = ActionEntry::builder("update-notes")
		.parameter_type(Some(&String::static_variant_type()))
		.activate(move |_, _action, parameter| {
			let path = parameter
				.expect("Could not get Parameter")
				.get::<String>()
				.expect("The variant nees to be of type `String`");
			notes_list_clone.borrow_mut().update_path(&path);
		})
		.build();

	window.add_action_entries([action_update_notes]);
}

fn register_open_note_action(window: &ApplicationWindow, editor: &Rc<RefCell<Editor>>) {
	let editor_clone = editor.clone();
	let action_open_notes = ActionEntry::builder("open-note")
		.parameter_type(Some(&String::static_variant_type()))
		.activate(move |_, _action, parameter| {
			let path = parameter
				.expect("Could not get Parameter")
				.get::<String>()
				.expect("The variant nees to be of type `String`");
			editor_clone.borrow_mut().update_path(&path);
		})
		.build();

	window.add_action_entries([action_open_notes]);
}
