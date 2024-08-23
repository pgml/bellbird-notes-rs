mod application;
pub mod default_layout;
pub mod directory_tree;
pub mod notes_list;
pub mod editor_view;
pub mod list_view_row;


fn main() -> gtk::glib::ExitCode {
	application::run()
}
