use bellbird_core::directories::Directories;

use gtk::{
	gio, prelude::*
};

use crate::list_view_row::ListViewItem;

fn create_tree_view() -> gtk::ListView {
	// let model = gio::ListStore::new<TreeItem::static_type()>();

	let bellbird_root = Directories::get_root_directory();
	let path = &bellbird_root;

	// let model = gio::ListStore::new::<ListViewItem>();
	let model = gio::ListStore::new::<gtk::Label>();
	match Directories::list(path, false) {
		Ok(directories) => {
			directories.iter().for_each(|directory| {
				let label = gtk::Label::builder()
					.label(&directory.name)
					.name(&directory.path)
					.build();

				// println!("{:?}", label);
				model.append(&label)
			})
		},
		_ => println!("No directories found")
	};

  let factory = gtk::SignalListItemFactory::new();
	factory.connect_setup(move |_factory, item| {
		let item = item.downcast_ref::<gtk::ListItem>().unwrap();
		let row = ListViewItem::default();
		item.set_child(Some(&row));
	});

	factory.connect_bind(move |_factory, item| {
		let item = item.downcast_ref::<gtk::ListItem>().unwrap();
		let label = item.item().and_downcast::<gtk::Label>().unwrap();
		let child = item.child().and_downcast::<ListViewItem>().unwrap();
		child.append_tree_item(&label);
	});

	// A sorter used to sort AppInfo in the model by their name
	let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
		let app_info1 = obj1.downcast_ref::<gtk::Label>().unwrap();
		let app_info2 = obj2.downcast_ref::<gtk::Label>().unwrap();

		app_info1
			.label()
			.to_lowercase()
			.cmp(&app_info2.label().to_lowercase())
			.into()
	});
	let sorted_model = gtk::SortListModel::new(Some(model), Some(sorter));
	let selection_model = gtk::SingleSelection::new(Some(sorted_model));

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

#[derive(Debug)]
pub struct DirectoryTree;

impl DirectoryTree {

}
