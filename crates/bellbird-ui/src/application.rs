use bellbird_core::directories::Directories;

use glib::closure_local;
use gtk::prelude::*;

use relm4::{
	prelude::*,
	typed_view::list::TypedListView
};

use crate::{
	default_layout, directory_tree::DirectoryTree, notes_list::{NotesList, NotesListItem}
};

#[derive(Debug)]
struct App {
	directories: gtk::TreeView,
	notes: TypedListView<NotesListItem, gtk::SingleSelection>,
}

#[derive(Debug)]
enum Msg {
	UpdateListBox(String),
}

#[relm4::component]
impl SimpleComponent for App {
	type Init = u8;
	type Input = Msg;
	type Output = ();

	view! {
		gtk::Window {
			set_title: Some("Bellbird Notes"),
			set_default_size: (1000, 600),
			set_css_classes: &["main-window"],

			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,

				gtk::Box {
					set_orientation: gtk::Orientation::Horizontal,
					set_spacing: 5,
					set_margin_all: 5,

					gtk::Box {
						set_orientation: gtk::Orientation::Vertical,
						set_vexpand: true,
						set_valign: gtk::Align::Fill,
						set_width_request: 200,
						set_css_classes: &["directories-panel"],

						gtk::Label {
							#[watch]
							set_label: &format!("Directories"),
							set_margin_all: 5,
							set_halign: gtk::Align::Start,
						},

						gtk::ScrolledWindow {
							set_hscrollbar_policy: gtk::PolicyType::Never,
							set_css_classes: &["scrolled-window"],

							#[local_ref]
							directory_tree -> gtk::TreeView {
								set_vexpand: true,
								set_valign: gtk::Align::Fill,
								set_activate_on_single_click: true,
								set_headers_visible: false,
							}
						}
					},

					gtk::Box {
						set_orientation: gtk::Orientation::Vertical,
						set_vexpand: true,
						set_valign: gtk::Align::Fill,
						set_width_request: 200,
						set_css_classes: &["notes-panel"],

						gtk::Label {
							#[watch]
							set_label: &format!("Notes"),
							set_margin_all: 5,
							set_halign: gtk::Align::Start,
						},

						gtk::ScrolledWindow {
							set_hscrollbar_policy: gtk::PolicyType::Never,

							#[local_ref]
							notes_list_box -> gtk::ListView {
								set_vexpand: true,
								set_valign: gtk::Align::Fill,
								set_margin_all: 5,
							}
						}
					},

					gtk::Box {
						set_orientation: gtk::Orientation::Vertical,
						set_css_classes: &["editor-panel"],

						gtk::Label {
							#[watch]
							set_label: &format!("Bellbird Notes"),
							set_margin_all: 5,
							set_halign: gtk::Align::Start,
						},

						gtk::ScrolledWindow {
							set_hscrollbar_policy: gtk::PolicyType::Never,

							gtk::TextView {
								set_left_margin: 10,
								set_right_margin: 10,
								set_top_margin: 10,
								set_bottom_margin: 10,
								set_indent: 10,
								set_wrap_mode: gtk::WrapMode::Word,
								set_vexpand: true,
								set_valign: gtk::Align::Fill,
								set_hexpand: true,
								set_halign: gtk::Align::Fill,
							},
						},
					}
				},

				// gtk::Box {
				// 	set_orientation: gtk::Orientation::Horizontal,
				// 	set_height_request: 30,
				// 	set_css_classes: &["status-bar"],
				// }
			}
		}
	}

	// Initialize the component.
	fn init(
		_: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let directories = DirectoryTree::build_tree_view();
		let directory_list_store = DirectoryTree::new().list_store;
		let notes: TypedListView<NotesListItem, gtk::SingleSelection> = NotesList::build_list_box("");

		let select = directories.selection();
		select.connect_closure("changed", false, closure_local!(move |selection: gtk::TreeSelection| {
			match selection.selected() {
				Some((model, tree_iter)) => {
					let selected_row = model.get::<String>(&tree_iter, 1);
					sender.input(Msg::UpdateListBox(selected_row));
				},
				None => println!("nothing's changed"),
			};
		}));


		let model = App {
			directories,
			notes,
		};

		let notes_list_box = &model.notes.view;
		let directory_tree = &model.directories;
		let widgets = view_output!();
		let bellbird_root = Directories::get_root_directory();

		widgets.directory_tree.set_model(Some(&directory_list_store));
		DirectoryTree::build(&directory_list_store, None, &bellbird_root);

		ComponentParts { model, widgets }
	}

	fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
		match msg {
			Msg::UpdateListBox(path) => {
				self.notes.clear();
				// self.notes = NotesList::build_list_box(&path);
				match bellbird_core::notes::Notes::list(&path) {
					Ok(notes) => {
						for note in notes {
							self.notes.append(NotesListItem::new(note.name))
						}
					},
					_ => println!("No notes found")
				}
			}
		}
	}
}


pub fn run() {
	let app = RelmApp::new("org.bellbird.notes");
	relm4::set_global_css(default_layout::DEFAULT_STYLE);

	// app.set_global_css_from_file("/home2/pgml/Projekte/bellbird_relm4/src/default.css");
	app.run::<App>(0);
}

