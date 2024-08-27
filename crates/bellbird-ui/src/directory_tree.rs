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
use crate::directory_tree_row::DirectoryTreeRow;

#[derive(Debug, Clone)]
pub struct DirectoryTree {
	pub path: PathBuf,
	pub model: gio::ListStore,
	pub list_view: gtk::ListView,
	pub current_directory: Rc<RefCell<PathBuf>>,
}

impl<'a> DirectoryTree {
	pub fn new(path: &'a Path) -> Self {
		let model = gio::ListStore::new::<gtk::Label>();
		//let model = gio::ListStore::new::<TreeItem>();
		let model_clone = model.clone();
		let config = Config::new();

	  let factory = gtk::SignalListItemFactory::new();
		factory.connect_setup(move |_factory, item| {
			let item = item.downcast_ref::<gtk::ListItem>().unwrap();
			let row = DirectoryTreeRow::default();
			item.set_child(Some(&row));
		});

		factory.connect_bind(move |_factory, item| {
			let item = item.downcast_ref::<gtk::ListItem>().unwrap();
			item.set_selectable(false);

			let label = item.item().and_downcast::<gtk::Label>().unwrap();
			let child = item.child().and_downcast::<DirectoryTreeRow>().unwrap();
			let dir_name = &label.label();
			let path = PathBuf::from(label.widget_name());
			let depth_from_root = Directories::get_depth_from_root(&path);

			//println!("{}", Directories::dir_has_children(&path));
			let has_children = Directories::dir_has_children(&path);
			child.append_tree_item(
				&dir_name,
				&path,
				depth_from_root,
				has_children
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

		let current_directory = Rc::new(RefCell::new(path.to_path_buf()));
		list_view.connect_activate(move |list_view, position| {
			let model = list_view.model().unwrap();
			let tree_item = model.item(position).and_downcast::<gtk::Label>().unwrap();
			let _name = tree_item.label();
			let path = tree_item.widget_name();
			model.select_item(position, true);

			config.set_value(
				ConfigSections::General,
				ConfigOptions::CurrentDirectory,
				path.to_string()
			);

			list_view
				.activate_action("win.update-notes", Some(&path.to_variant()))
				.expect("The action `update-notes` does not exist.");
		});

		Self {
			path: path.to_path_buf(),
			model,
			list_view,
			current_directory,
		}
	}

	pub fn update_path(&mut self, path: PathBuf) {
		self.path = path.clone();
		self.model.remove_all();
		self.append_to_model(&path);
		self.set_selection();
	}

	fn append_to_model(&self, path: &Path) {
		if let Ok(directories) = Directories::list(&path, true) {
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
				self.append_to_model(&directory.path);
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
}

pub fn build_ui(directory_tree: Rc<RefCell<DirectoryTree>>) -> gtk::Box {
	let directory_panel = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.vexpand(true)
		.valign(gtk::Align::Fill)
		.width_request(190)
		.css_classes(["directories-panel"])
		.build();

	let directory_panel_label = gtk::Label::builder()
		.label("Folders")
		.margin_start(10)
		.margin_end(10)
		.margin_top(5)
		.margin_bottom(5)
		.halign(gtk::Align::Start)
		.build();

	let scrollable_window = gtk::ScrolledWindow::builder()
		.child(directory_tree.borrow_mut().view())
		.hscrollbar_policy(gtk::PolicyType::External)
		.build();

	let handle_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
	handle_box.append(&gtk::WindowControls::new(gtk::PackType::Start));
	let _window_handle = gtk::WindowHandle::builder()
		.child(&handle_box)
		.build();

	//directory_panel.append(&_window_handle);
	directory_panel.append(&directory_panel_label);
	directory_panel.append(&scrollable_window);

	directory_panel
}
