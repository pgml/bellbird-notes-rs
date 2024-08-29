// Turns off console window on Windows, but not when building with dev profile.
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use anyhow::Context;
use gtk::gio;

mod application;
pub mod default_layout;
pub mod directory_tree;
pub mod notes_list;
pub mod editor_view;
pub mod directory_tree_row;
pub mod notes_list_row;
pub mod breadcrumb;
pub mod action_entries;
pub mod contextmenu;
pub mod menu;
pub mod dialogue;

fn main() -> gtk::glib::ExitCode {
	if let Err(e) = setup_gresources() {
		eprintln!("failed to setup gresources, Err: {e:?}");
	}

	application::run()
}

fn setup_gresources() -> anyhow::Result<()> {
	gio::resources_register_include!("compiled.gresource")
		.context("Failed to register and include compiled gresource.")
}
