use bellbird_core::directories::Directories;

use gtk::{
	gio, prelude::*
};

use crate::directory_tree_row::DirectoryTreeRow;

fn append_item_to_model(model: &gio::ListStore, path: &str) {
	match Directories::list(path, true) {
		Ok(directories) => {
			directories.iter().for_each(|directory| {
				let dir_name = directory.name.clone();

				let label = gtk::Label::builder()
					.label(&dir_name)
					.name(&directory.path)
					.build();

				model.append(&label);
				//append_item_to_model(model, &directory.path);
			})
		},
		_ => println!("No directories found")
	};
}

fn create_tree_view() -> gtk::ListView {
	let bellbird_root = Directories::root_directory();
	let path = &bellbird_root;
	let model = gio::ListStore::new::<gtk::Label>();

	append_item_to_model(&model, path);

  let factory = gtk::SignalListItemFactory::new();
	factory.connect_setup(move |_factory, item| {
		let item = item.downcast_ref::<gtk::ListItem>().unwrap();
		let row = DirectoryTreeRow::new();
		item.set_child(Some(&row));
	});

	factory.connect_bind(move |_factory, item| {
		let item = item.downcast_ref::<gtk::ListItem>().unwrap();
		let label = item.item().and_downcast::<gtk::Label>().unwrap();
		let child = item.child().and_downcast::<DirectoryTreeRow>().unwrap();
		child.append_tree_item(&label);
	});

	// A sorter used to sort AppInfo in the model by their name
	let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
		let app_info1 = obj1.downcast_ref::<gtk::Label>().unwrap();
		let _app_info2 = obj2.downcast_ref::<gtk::Label>().unwrap();

		app_info1
			.label()
			.to_lowercase()
			.cmp(&app_info1.label().to_lowercase())
			.into()
	});
	let sorted_model = gtk::SortListModel::new(Some(model), Some(sorter));
	let selection_model = gtk::SingleSelection::new(Some(sorted_model));
	selection_model.set_autoselect(false);

	let list_view = gtk::ListView::builder()
		.model(&selection_model)
		.factory(&factory)
		.vexpand(true)
		.valign(gtk::Align::Fill)
		.single_click_activate(true)
		.build();

	list_view.connect_activate(move |list_view, position| {
		let model = list_view.model().unwrap();
		let tree_item = model.item(position).and_downcast::<gtk::Label>().unwrap();

		let _name = tree_item.label();
		let path = tree_item.widget_name();

		list_view
			.activate_action("win.update-notes", Some(&path.to_variant()))
			.expect("The action `update-notes` does not exist.");
	});

	list_view
}

pub fn build_ui() -> gtk::Box {
	let directory_panel = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.vexpand(true)
		.valign(gtk::Align::Fill)
		.width_request(200)
		.margin_top(5)
		.margin_bottom(5)
		.margin_start(5)
		.css_classes(["directories-panel"])
		.build();

	let directory_panel_label = gtk::Label::builder()
		.label("Directories")
		.margin_top(5)
		.margin_start(5)
		.margin_bottom(5)
		.halign(gtk::Align::Start)
		.build();

	let scrollable_window = gtk::ScrolledWindow::builder()
		.child(&create_tree_view())
		.build();

	directory_panel.append(&directory_panel_label);
	directory_panel.append(&scrollable_window);

	directory_panel
}
