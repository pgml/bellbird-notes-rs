use bellbird_core::notes::Notes;
use relm4::{
	binding::StringBinding,
	gtk,
	typed_view::list::{
		RelmListItem,
		TypedListView
	},
	RelmWidgetExt
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotesListItem {
	pub name: String,
	pub binding: StringBinding,
}

impl NotesListItem {
	pub fn new(name: String) -> Self {
		Self {
			name,
			binding: StringBinding::new(String::new()),
		}
	}
}

pub struct Widgets {
	pub label: gtk::Label,
}

impl RelmListItem for NotesListItem {
	type Root = gtk::Box;
	type Widgets = Widgets;

	fn setup(_item: &gtk::ListItem) -> (gtk::Box, Widgets) {
		relm4::view! {
			my_box = gtk::Box {
				#[name = "label"]
				gtk::Label,
			}
		}

		let widgets = Widgets {
			label,
		};

		(my_box, widgets)
	}

	fn bind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
		let Widgets {
			label,
		} = widgets;

		label.set_margin_all(5);
		label.set_label(&self.name);
	}
}

#[derive(Debug)]
pub struct NotesList;

impl NotesList {
	pub fn build_list_box(path: &str) -> TypedListView<NotesListItem, gtk::SingleSelection> {
		let mut notes_list = TypedListView::with_sorting();

		match Notes::list(path) {
			Ok(notes) => {
				for note in notes {
					notes_list.append(NotesListItem::new(note.name))
				}
			},
			_ => println!("No notes found")
		}

		notes_list
	}

	pub fn update_list_box(
		path: &str,
		mut list_box: TypedListView<NotesListItem, gtk::SingleSelection>
	) -> TypedListView<NotesListItem, gtk::SingleSelection> {
		match Notes::list(path) {
			Ok(notes) => {
				for note in notes {
					list_box.append(NotesListItem::new(note.name))
				}
			},
			_ => println!("No notes found")
		}

		list_box
	}
}
