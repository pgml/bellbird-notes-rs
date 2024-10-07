use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use gtk::{gio, prelude::*};
use gtk::prelude::WidgetExt;

use bellbird_core::config::{
	Config,
	ConfigOptions,
	ConfigSections
};

use bellbird_core::directories::Directories;
use crate::contextmenu::{BbMenuItem, BbMenuSection, ContextMenu};
use crate::directory_tree_row::DirectoryTreeRow;

#[derive(Debug, Clone)]
pub struct TreeItem<'a> {
	pub name: &'a str,
	pub path: PathBuf
}

#[derive(Debug, Clone)]
pub struct DirectoryTree {
	pub path: PathBuf,
	pub model: gio::ListStore,
	pub list_view: gtk::ListView,
	pub current_directory: Rc<RefCell<PathBuf>>,
	pub selected_ctx_path: Rc<RefCell<PathBuf>>,
}

impl<'a> DirectoryTree {
	pub fn new(path: &'a Path) -> Self {
		let model = gio::ListStore::new::<gtk::Label>();
		let model_clone = model.clone();

	  let factory = gtk::SignalListItemFactory::new();
		factory.connect_setup(move |_factory, item| {
			let item = item.downcast_ref::<gtk::ListItem>().unwrap();
			let row = DirectoryTreeRow::default();
			//let expander = gtk::TreeExpander::new();
			//let expander_icon = gtk::Image::builder()
			//	.resource("/com/bellbird/notes/icons/arrow-right.svg")
			//	.pixel_size(12)
			//	.build();
			//expander.set_child(Some(&expander_icon));
			////row.set_expander(&expander);
			item.set_child(Some(&row));
		});

		factory.connect_bind(move |_factory, item| {
			let item = item.downcast_ref::<gtk::ListItem>().unwrap();
			item.set_selectable(false);
			let label = item.item().and_downcast::<gtk::Label>().unwrap();
			let child = item.child().and_downcast::<DirectoryTreeRow>().unwrap();
			let dir_name = &label.label();
			let path = PathBuf::from(label.widget_name());
			//let depth_from_root = Directories::get_depth_from_root(&path);

			//let has_children = Directories::dir_has_children(&path);
			child.append_tree_item(
				&TreeItem { name: dir_name, path },
				//&dir_name,
				//&path,
				//depth_from_root,
				//has_children
			);
		});

		let selection_model = gtk::MultiSelection::new(Some(model_clone));

		let list_view = gtk::ListView::builder()
			.model(&selection_model)
			.factory(&factory)
			.vexpand(true)
			.valign(gtk::Align::Fill)
			.margin_top(5)
			.margin_bottom(5)
			.single_click_activate(true)
			.build();

		list_view.connect_activate(move |list_view, position| {
			let model = list_view.model().unwrap();
			let tree_item = model.item(position).and_downcast::<gtk::Label>().unwrap();
			let _name = tree_item.label();
			let path = tree_item.widget_name();
			model.select_item(position, true);

			let _ = Config::new().set_config_value(
				ConfigSections::General.as_str(),
				ConfigOptions::CurrentDirectory,
				path.to_string()
			);

			list_view
				.activate_action("app.refresh-notes", Some(&path.to_variant()))
				.expect("The action `refresh-notes` does not exist.");
		});

		Self {
			path: path.to_path_buf(),
			model,
			list_view,
			current_directory: Rc::new(RefCell::new(path.to_path_buf())),
			selected_ctx_path: Rc::new(RefCell::new(path.to_path_buf())),
		}
	}

	pub fn update_path(&mut self, path: PathBuf) {
		self.path = path.clone();
		self.model.remove_all();
		self.append_to_model(&path);
		self.set_selection();
	}

	pub fn refresh(&mut self) {
		self.update_path(self.path.clone());
	}

	fn append_to_model(&self, path: &Path) {
		if let Some(directories) = Directories::list(&path, 1) {
			directories.iter().for_each(|directory| {
				let dir_name = directory.name.clone();
				let path = directory.path.display().to_string();

				//let tree_item = TreeItem::new();
				//tree_item.set_name(Some(dir_name));
				//tree_item.set_path(Some(path));
				//self.model.append(&tree_item);

				//let row = DirectoryTreeRow::new();
				//row.set_name(Some(dir_name));
				//row.set_path(Some(directory.path.display().to_string()));

				let label = gtk::Label::builder()
					.label(&dir_name)
					.name(&path)
					.build();
				self.model.append(&label);
				//self.append_to_model(&directory.path);
			})
		}
	}

	fn view(&self) -> &gtk::ListView {
		&self.list_view
	}

	pub fn update_current_directory(&self, path: PathBuf) {
		self.current_directory.borrow_mut().set_file_name(path);
	}

	fn set_selection(&self) {
		let current_directory = self.current_directory.clone();
		if let Some(selection_model) = self.list_view.model() {
			for index in 0..selection_model.n_items() {
				if let Some(item) = selection_model.item(index) {
					let path = item.downcast::<gtk::Label>()
						.unwrap()
						.widget_name();

					let current_directory = current_directory
						.borrow_mut()
						.display()
						.to_string();

					if path.to_string() == current_directory {
						selection_model.select_item(index, true);
						break;
					}
				}
			}
		}
	}

	pub fn set_selected_ctx_note(&self, path: PathBuf) {
		self.selected_ctx_path.borrow_mut().set_file_name(path);
	}

	fn build_context_menu(&self, app: &adw::Application) {
		let mut sections = vec![];
		//let mut sec0 = vec![];
		//sec0.push(BbMenuItem { label: "Open in New Tab", action: "open-note-in-tab" });
		//sections.push(BbMenuSection { label: None, items: sec0 });

		let mut sec1 = vec![];
		sec1.push(BbMenuItem { label: "Create Folder", action: "create-folder" });
		sections.push(BbMenuSection { label: None, items: sec1 });

		let mut sec2 = vec![];
		//sec2.push(BbMenuItem { label: "Duplicate Folder", action: "duplicate-folder" });
		//sec2.push(BbMenuItem { label: "Pin / Unpin Folder", action: "toggle-pin-folder" });
		sec2.push(BbMenuItem { label: "Rename Folder", action: "rename-folder" });
		sections.push(BbMenuSection { label: None, items: sec2 });

		let mut sec3 = vec![];
		sec3.push(BbMenuItem { label: "Delete Folder", action: "delete-folder" });
		sections.push(BbMenuSection { label: None, items: sec3 });

		let app_clone = app.clone();
		let self_clone = self.clone();

		let list_view = vec![self.list_view.clone()];
		ContextMenu::new(sections, list_view, 180).build(move |widget| {
			let actions = vec![
				"open-folder-in-tab",
				"duplicate-folder",
				"toggle-pin-folder",
				"rename-folder",
				"delete-folder"
			];
			for action in actions.iter() {
				app_clone.action_enabled_changed(action, false);
			}

			if widget.widget_name() != "GtkListView" {
				#[allow(unused)]
				let mut should_activate_on_folder_items = false;
				let mut directory_path = PathBuf::from("");

				if widget.widget_name() == "DirectoryTreeRow" {
					should_activate_on_folder_items = true;
					if let Some(label) = widget.first_child().unwrap().last_child().and_downcast::<gtk::Label>() {
						directory_path.push(label.label());
					}
				}
				else {
					should_activate_on_folder_items = true;
					if let Some(parent) = widget.parent() {
						if let Some(label) = parent.last_child().and_downcast::<gtk::Label>() {
							directory_path.push(label.label());
						}
					}
				}

				self_clone.set_selected_ctx_note(directory_path.clone().into());
				//println!("{:?}", directory_path);
				if !directory_path.as_os_str().is_empty() && should_activate_on_folder_items {
					for action in actions.iter() {
						app_clone.action_enabled_changed(action, true);
					}
				}
			}
		});
	}
}


pub fn build_ui(
	app: &adw::Application,
	directory_tree: &Rc<RefCell<DirectoryTree>>
) -> gtk::Box {
	let directory_panel = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.vexpand(true)
		.valign(gtk::Align::Fill)
		.width_request(190)
		.name("directories-tree")
		.css_classes(["directories-panel"])
		.build();

	let directory_panel_label = gtk::Label::builder()
		.label("Folders")
		.margin_start(12)
		.margin_end(10)
		.margin_top(5)
		.margin_bottom(5)
		.halign(gtk::Align::Start)
		.build();

	let scrollable_window = gtk::ScrolledWindow::builder()
		.child(directory_tree.borrow_mut().view())
		.hscrollbar_policy(gtk::PolicyType::External)
		.build();

	directory_tree.borrow_mut().build_context_menu(app);

	//let handle_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
	//handle_box.append(&gtk::WindowControls::new(gtk::PackType::Start));
	//let _window_handle = gtk::WindowHandle::builder()
	//	.child(&handle_box)
	//	.build();

	//handle_box.append(&directory_panel_label);
	//directory_panel.append(&_window_handle);
	directory_panel.append(&directory_panel_label);
	directory_panel.append(&scrollable_window);

	directory_panel
}
