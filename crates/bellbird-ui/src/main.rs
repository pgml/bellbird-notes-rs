mod application;
pub mod directory_tree;
pub mod notes_list;
pub mod default_layout;

// fn load_stylesheet() {
// 	let provider = CssProvider::new();
// 	provider.load_from_resource("/home2/pgml/Projekte/bellbird_rel pv-m4/src/default.css");
// 	gtk::style_context_add_provider_for_display(
// 		&Display::default().expect("Could not connect to a display."),
// 		&provider,
// 		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
// 	);
// }

fn main() {
	application::run();
}
