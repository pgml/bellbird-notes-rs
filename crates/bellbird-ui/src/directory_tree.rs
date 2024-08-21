// use glib::closure_local;
use gtk::{
	prelude::*,
	CellRendererText,
	// TreeSelection,
	TreeStore,
	TreeView,
	TreeViewColumn
};

use relm4::prelude::*;

use bellbird_core::directories::Directories;

#[derive(Debug)]
pub struct DirectoryTree {
	pub list_store: TreeStore
}

impl DirectoryTree {
	pub fn new() -> Self {
		let list_store = TreeStore::new(&[
				String::static_type(),
				String::static_type(),
		]);

		Self {
			list_store
		}
	}

	pub fn build(
		store: &gtk::TreeStore,
		parent_iter: Option<&gtk::TreeIter>,
		path: &str
	) {
		match Directories::list(path, false) {
			Ok(directories) => {
				for directory in directories {
					let name = directory.name;
					let path = directory.path;
					let iter = store.append(parent_iter);

					store.set(&iter, &[(0, &name), (1, &path)]);
					Self::build(store, Some(&iter), &path);
				}
			},
			_ => println!("No directories found")
		};
	}

	pub fn build_tree_view() -> TreeView {
		let tree_view: TreeView = TreeView::new();
		tree_view.set_margin_top(5);
		tree_view.set_enable_tree_lines(true);

		let name_renderer = CellRendererText::new();
		name_renderer.set_padding(0, 2);
		// let icon_renderer = CellRendererPixbuf::new();
		let name_column = TreeViewColumn::new();
		name_column.set_title("name");
		name_column.pack_start(&name_renderer, true);
		name_column.add_attribute(&name_renderer, "text", 0);
		tree_view.append_column(&name_column);

		// hide path column
		// need it for getting all the notes in given path
		let path_renderer = CellRendererText::new();
		// let icon_renderer = CellRendererPixbuf::new();
		let path_column = TreeViewColumn::new();
		path_column.set_title("path");
		path_column.pack_start(&path_renderer, true);
		path_column.add_attribute(&path_renderer, "text", 1);
		path_column.set_visible(false);
		tree_view.append_column(&path_column);

		tree_view
	}
}
