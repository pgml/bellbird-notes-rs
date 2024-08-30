use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc};

use bellbird_core::directories::Directories;
use gtk::{gio, prelude::*};

use crate::{dialogue::Dialogue, directory_tree::DirectoryTree};


#[derive(Debug, Clone)]
pub struct DirectoryTreeContextMenu {
	app: adw::Application,
	directory_tree
	: Rc<RefCell<DirectoryTree>>,
}

impl DirectoryTreeContextMenu {
	pub fn new(
		app: &adw::Application,
		directory_tree: Rc<RefCell<DirectoryTree>>
	) -> Self {
		Self {
			app: app.clone(),
			directory_tree
		}
	}

	pub fn setup_context_menu_actions(self: Arc<Self>) {
		let app_clone = self.app.clone();

		let open_in_tab = gio::SimpleAction::new("open-folder-in-tab", None);
		open_in_tab.connect_activate(move |_, _| println!("open in tab"));
		app_clone.add_action(&open_in_tab);

		// @todo try to make this whole thing less verbose
		let create_folder = gio::SimpleAction::new("create-folder", None);
		{
			let self_clone = Arc::clone(&self);
			create_folder.connect_activate(move |_, _| self_clone.create_folder());
		}
		app_clone.add_action(&create_folder);

		let duplicate_folder = gio::SimpleAction::new("duplicate-folder", None);
		duplicate_folder.connect_activate(move |_, _| println!("duplicate folder"));
		app_clone.add_action(&duplicate_folder);

		let pin_folder = gio::SimpleAction::new("toggle-pin-folder", None);
		pin_folder.connect_activate(move |_, _| println!("pin folder"));
		app_clone.add_action(&pin_folder);

		let rename_folder = gio::SimpleAction::new("rename-folder", None);
		{
			let self_clone = Arc::clone(&self);
			rename_folder.connect_activate(move |_, _| self_clone.rename_folder());
		}
		app_clone.add_action(&rename_folder);

		let delete_folder = gio::SimpleAction::new("delete-folder", None);
		{
			let self_clone = Arc::clone(&self);
			delete_folder.connect_activate(move |_, _| self_clone.delete_folder());
		}
		app_clone.add_action(&delete_folder);
	}

	fn create_folder(&self) {
		let directory_tree_clone = self.directory_tree.clone();
		let dialogue = Dialogue::new(&self.app);
		dialogue.input(
			"Create New Folder",
			"Enter folder name:",
			"New Folder",
			move |folder| {
				let mut path = PathBuf::from(directory_tree_clone.borrow_mut()
					                           .path.to_str().unwrap_or(""));
				path.push(&folder);
				Directories::create(&path);
				directory_tree_clone.borrow_mut().refresh();
			},
			|| {}
		)
	}

	fn rename_folder(&self) {
		let directory_tree_clone = self.directory_tree.clone();
		let dialogue = Dialogue::new(&self.app);
		let pathbuf_rc = self.directory_tree.borrow_mut().selected_ctx_path.clone();
		let (full_path, _, file_stem) = self.get_path_and_stem(&pathbuf_rc);
		dialogue.input(
			"Rename Folder",
			&format!("Rename ´{}´ to:", file_stem),
			&file_stem,
			move |folder| {
				let mut new_path = PathBuf::from(directory_tree_clone.borrow_mut()
					                           .path.to_str().unwrap_or(""));
				new_path.push(&folder);
				let old_path = PathBuf::from(&full_path);
				Directories::rename(&old_path, &new_path);
				directory_tree_clone.borrow_mut().refresh();
			},
			|| {}
		)
	}

	fn delete_folder(&self) {
		// this whole thing is pretty ugly
		// but works for now
		let app_clone = self.app.clone();
		let directory_tree_clone = self.directory_tree.clone();
		let dialogue = Dialogue::new(&app_clone);
		let pathbuf_rc = self.directory_tree.borrow_mut().selected_ctx_path.clone();
		let (full_path, directory_path, _) = self.get_path_and_stem(&pathbuf_rc);
		dialogue.warning_yes_no(
			"Delete Folder",
			"Do you really want to delete this folder?\n(Note: its content will also be deleted)",
			&format!("´{}´", directory_path),
			move || {
				Directories::delete(&PathBuf::from(&full_path), true);
				directory_tree_clone.borrow_mut().refresh();
			},
			|| {}
		)
	}

	fn get_path_and_stem(&self, path: &Rc<RefCell<PathBuf>>) -> (String, String,  String) {
		let directory_path = path.borrow_mut();
		let file_stem = directory_path.file_stem().unwrap()
			              .to_str().unwrap().to_string();
		let bellbird_root = Directories::root_directory().display().to_string();
		let full_path = directory_path.display().to_string();
		let directory_path = full_path.replace(&bellbird_root, "");
		(full_path, directory_path, file_stem)
	}
}
