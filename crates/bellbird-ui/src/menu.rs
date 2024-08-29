// temporary
//pub fn _menubar(app: &adw::Application) {
//	let about = gtk::gio::ActionEntry::builder("about")
//		.activate(|_, _, _| println!("About was pressed"))
//		.build();
//
//	let quit = gtk::gio::ActionEntry::builder("quit")
//		.activate(|app: &adw::Application, _, _| app.quit())
//		.build();
//
//	app.add_action_entries([about, quit]);
//
//	let menubar = {
//		let file_menu = {
//			let about_menu_item = gtk::gio::MenuItem::new(Some("About"), Some("app.about"));
//			let quit_menu_item = gtk::gio::MenuItem::new(Some("Quit"), Some("app.quit"));
//
//			let file_menu = gtk::gio::Menu::new();
//			file_menu.append_item(&about_menu_item);
//			file_menu.append_item(&quit_menu_item);
//			file_menu
//		};
//
//		let menubar = gtk::gio::Menu::new();
//		menubar.append_submenu(Some("File"), &file_menu);
//
//		menubar
//	};
//
//	app.set_menubar(Some(&menubar));
//}
