mod application;
pub mod default_layout;
pub mod directory_tree;
pub mod notes_list;
pub mod editor_view;
pub mod directory_tree_row;
pub mod notes_list_row;


fn main() -> gtk::glib::ExitCode {
	application::run()
}
