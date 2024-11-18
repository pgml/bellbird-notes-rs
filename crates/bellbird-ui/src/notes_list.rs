use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use bellbird_core::config::{Config, ConfigOptions, ConfigSections};
use bellbird_core::notes::Notes;

use glib::MainContext;
use gtk::gio;
use gtk::prelude::*;

use crate::contextmenu::{BbMenuItem, BbMenuSection, ContextMenu};
use crate::notes_list_model_item::ListModelItem;
use crate::notes_list_row::NotesListItem;


#[derive(Debug, Clone)]
pub struct NotesList {
	pub path: PathBuf,
	pub model: (gio::ListStore, gio::ListStore),
	pub list_view: (gtk::ListView, gtk::ListView),
	pub current_note: Rc<RefCell<PathBuf>>,
	pub selected_ctx_path: Rc<RefCell<PathBuf>>,
	panel_box: gtk::Box
}

impl<'a> NotesList {
	pub fn new(path: &'a Path) -> Self {
		let factory = Self::create_list_view_factory();
		let (list_view, model) = Self::create_list_view(
			factory,
			true,
			gtk::Align::Fill
		);

		let factory_pinned = Self::create_list_view_factory();
		let (list_view_pinned, model_pinned) = Self::create_list_view(
			factory_pinned,
			true,
			gtk::Align::Start
		);

		let panel_box = NotesList::panel_box();

		Self {
			path: path.to_path_buf(),
			model: (model, model_pinned),
			list_view: (list_view, list_view_pinned),
			current_note: Rc::new(RefCell::new(path.to_path_buf())),
			selected_ctx_path: Rc::new(RefCell::new(path.to_path_buf())),
			panel_box
		}
	}

	pub async fn update_path(&mut self, path: PathBuf) -> Option<bool> {
		self.path = path.clone();
		let (model, model_pinned) = &self.model;
		let this = self.clone();

		// hide pinned notes when there are none
		// @todo: try to remove the flickering when switching to directories
		// with pinned notes from a directory without pinned notes
		let first_child = self.panel_box.first_child().unwrap();
		first_child.set_visible(false);

		let pinned_notes = Notes::pinned_notes(&self.path).unwrap();
		if !pinned_notes.is_empty() {
			first_child.set_visible(true);
		}

		MainContext::default().spawn_local(glib::clone!(
			#[weak] model, #[weak] model_pinned, #[strong] this,
			async move {
				if let Ok(notes) = Notes::list(&path).await {
					model.remove_all();
					model_pinned.remove_all();
					notes.iter().for_each(|note| {
						let path = note.path.clone();
						let list_item = ListModelItem::new();
						list_item.set_name(&note.name);
						list_item.set_path(&path);
						//list_item.set_is_pinned(note.is_pinned);
						if note.is_pinned {
							model_pinned.append(&list_item);
						}
						else {
							model.append(&list_item);
						}
					});
					this.set_selection();
				}
			}
		));

		Some(true)
	}

	fn create_list_view(
		factory: gtk::SignalListItemFactory,
		vexpand: bool,
		valign: gtk::Align,
	) -> (gtk::ListView, gio::ListStore) {
		//let model = gio::ListStore::new::<gtk::Label>();
		let model = gio::ListStore::new::<ListModelItem>();

		let selection_model = gtk::MultiSelection::new(Some(model.clone()));
		let list_view = gtk::ListView::builder()
			.model(&selection_model)
			.factory(&factory)
			.vexpand(vexpand)
			.valign(valign)
			.margin_top(5)
			.margin_bottom(5)
			//.margin_start(5)
			//.margin_end(5)
			.single_click_activate(true)
			.show_separators(true)
			.build();

		list_view.connect_activate(
			move |list_view, position| {
				let model = list_view.model().unwrap();
				let model_item = model.item(position).and_downcast::<ListModelItem>().unwrap();
				let path = model_item.path();
				//path = label.path();
				//println!("{:?} {:?}", model_item, path);
				model.select_item(position, true);

				let _ = Config::new().set_config_value(
					ConfigSections::General.as_str(),
					ConfigOptions::CurrentNote,
					path.to_string()
				);

				list_view
					.activate_action("app.open-note", Some(&path.to_variant()))
					.expect("The action `open-note` does not exist.");
			}
		);

		(list_view, model)
	}

	fn create_list_view_factory() -> gtk::SignalListItemFactory {
		let factory = gtk::SignalListItemFactory::new();
		factory.connect_setup(move |_factory, item| {
			let item = item.downcast_ref::<gtk::ListItem>().unwrap();
			let row = NotesListItem::default();
			item.set_child(Some(&row));
		});

		factory.connect_bind(move |_factory, item| {
			let item = item.downcast_ref::<gtk::ListItem>().unwrap();
			//let label = item.item().and_downcast::<gtk::Label>().unwrap();
			let model_item = item.item().and_downcast::<ListModelItem>().unwrap();
			let child = item.child().and_downcast::<NotesListItem>().unwrap();
			item.set_selectable(false);

			child.append_tree_item(
				&model_item.name(),
				model_item.path().into(),
				false,
			);
		});

		factory
	}

	pub async fn refresh(&mut self) {
		self.update_path(self.path.clone()).await;
	}

	pub fn update_current_note(&self, path: PathBuf) {
		self.current_note.borrow_mut().set_file_name(path);
	}

	pub fn set_selected_ctx_note(&self, path: PathBuf) {
		self.selected_ctx_path.borrow_mut().set_file_name(path);
	}

	fn view(&self) -> (&gtk::ListView, &gtk::ListView) {
		let (list_view, list_view_pinned) = &self.list_view;
		(&list_view, &list_view_pinned)
	}

	fn set_selection(&self) {
		let current_note = self.current_note.clone();
		let (list_view, _list_view_pinned) = &self.list_view;
		//let list_view = &self.list_view;

		if let Some(selection_model) = list_view.model() {
			for index in 0..selection_model.n_items() {
				let Some(item) = selection_model.item(index) else {
					continue;
				};

				let path = item.downcast::<ListModelItem>().unwrap().path();
				if path.to_string() == current_note.borrow_mut().display().to_string() {
					selection_model.select_item(index, true);
					break;
				}
			}
		}
	}

	fn build_context_menu(&self, app: &adw::Application) {
		let mut sections = vec![];
		//let mut sec0 = vec![];
		//sec0.push(BbMenuItem { label: "Open in New Tab", action: "open-note-in-tab" });
		//sections.push(BbMenuSection { label: None, items: sec0 });

		let mut sec1 = vec![];
		sec1.push(BbMenuItem { label: "Create Note", action: "create-note" });
		sections.push(BbMenuSection { label: None, items: sec1 });

		let mut sec2 = vec![];
		sec2.push(BbMenuItem { label: "Duplicate Note", action: "duplicate-note" });
		sec2.push(BbMenuItem { label: "Pin / Unpin Note", action: "toggle-pin-note" });
		sec2.push(BbMenuItem { label: "Rename Note", action: "rename-note" });
		sections.push(BbMenuSection { label: None, items: sec2 });

		let mut sec3 = vec![];
		sec3.push(BbMenuItem { label: "Delete Note", action: "delete-note" });
		sections.push(BbMenuSection { label: None, items: sec3 });

		let app_clone = app.clone();
		let self_clone = self.clone();

		let (list_view, list_view_pinned) = &self.list_view;
		let list_views = vec![list_view.clone(), list_view_pinned.clone()];

		ContextMenu::new(sections, list_views, 180).build(move |widget| {
			let actions = vec![
				"open-note-in-tab",
				"duplicate-note",
				"toggle-pin-note",
				"rename-note",
				"delete-note"
			];

			for action in actions.iter() {
				app_clone.action_enabled_changed(action, false);
			}

			if widget.widget_name() != "GtkListView" {
				match widget.parent() {
					Some(parent) => {
						self_clone.handle_context_menu_parent(&app_clone, &parent, &actions);
					},
					None => println!("nope")
				}
			}
		});
	}

	fn handle_context_menu_parent(
		&self,
		app: &adw::Application,
		parent: &gtk::Widget,
		actions: &Vec<&str>
	) {
		#![allow(unused)]
		let mut should_activate_on_note_items = false;
		let mut path = PathBuf::from("");

		if parent.widget_name() == "NotesListRow" {
			should_activate_on_note_items = true;
			if let Some(label) = parent.last_child().and_downcast::<gtk::Label>() {
				path.push(label.label());
			}
		}
		else {
			should_activate_on_note_items = true;
			if let Some(first_child) = parent.first_child() {
				if let Some(label) = first_child.last_child().and_downcast::<gtk::Label>() {
					path.push(label.label());
				}
			}
		}

		self.set_selected_ctx_note(path.clone().into());
		if !path.as_os_str().is_empty() && should_activate_on_note_items {
			for action in actions.iter() {
				app.action_enabled_changed(action, true);
			}
		}
	}

	fn _enable_actions(&self, app: &adw::Application, actions: Vec<&str>) {
		for action in actions.iter() {
			app.action_enabled_changed(action, true);
		}
	}

	fn _disable_actions(&self, app: adw::Application, actions: Vec<&str>) {
		for action in actions.iter() {
			app.action_enabled_changed(action, false);
		}
	}

	fn panel_box() -> gtk::Box {
		let notes_panel = gtk::Box::builder()
			.orientation(gtk::Orientation::Vertical)
			.vexpand(true)
			.valign(gtk::Align::Fill)
			.width_request(195)
			.margin_top(3)
			.margin_bottom(3)
			.margin_end(2)
			.css_classes(["notes-panel"])
			.build();

		notes_panel
	}

	pub fn build_ui(
		&self,
		app: &adw::Application,
	) -> gtk::Box {
		let (view, pinned_view) = self.view();

		let notes_view_pinned = create_list_view_wrapper(
			"Pinned",
			pinned_view,
			false,
			glib::clone!(#[weak] pinned_view,
				move |wrapper, _scrolled_window, _| {
					let (min_size, _) = pinned_view.preferred_size();
					wrapper.set_height_request(min_size.height());
					//println!("{:?} {:?}", pinned_view.height_request(), view.height());
					//view.set_height_request(min_size.height());
					//view.set_height_request(min_size.height());
					//println!("{:?}", view.preferred_size());
				}
			)
		);

		let notes_view = create_list_view_wrapper(
			"Notes",
			view,
			true,
			|_, _, _| {}
		);

		self.build_context_menu(app);
		self.panel_box.append(&notes_view_pinned);
		self.panel_box.append(&notes_view);
		self.panel_box.clone()
	}
}

fn create_list_view_wrapper<F>(
	label: &str,
	view: &gtk::ListView,
	vexpand: bool,
	f: F
) -> gtk::Box
where
	F: Fn(gtk::Box, gtk::ScrolledWindow, gtk::Label) + 'static + Clone
{
	let notes_panel_label = gtk::Label::builder()
		.label(label)
		.margin_start(10)
		.margin_end(5)
		.margin_top(5)
		.margin_bottom(5)
		.halign(gtk::Align::Start)
		.valign(gtk::Align::Start)
		.build();

	let scrollable_window = gtk::ScrolledWindow::builder()
		.hscrollbar_policy(gtk::PolicyType::External)
		.child(view)
		.build();

	let wrapper = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.name(label.to_lowercase())
		.vexpand(vexpand)
		//.valign(gtk::Align::Fill)
		.build();

	wrapper.append(&notes_panel_label);
	wrapper.append(&scrollable_window);

	view.model().unwrap().connect_items_changed(glib::clone!(
		#[weak] wrapper, #[weak] scrollable_window, #[weak] notes_panel_label,
		move |_model, _, _, _| {
			f(wrapper.clone(), scrollable_window, notes_panel_label);
		}
	));

	wrapper
}
