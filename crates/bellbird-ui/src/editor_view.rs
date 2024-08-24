use std::{cell::RefCell, rc::Rc};

use gtk::{gio, glib, prelude::*};
use sourceview5::{self, Buffer, View};

#[derive(Debug, Clone)]
pub struct Editor {
	pub path: String,
	//pub buffers: Vec<Buffer>,
	pub buffer: Buffer,
	pub editor_view: View,
}

impl Editor	{
	pub fn new(path: &str) -> Self {
		let buffer = sourceview5::Buffer::new(None);
		// let editor_view = View::builder().build();

		let editor_view = View::new();
		editor_view.set_top_margin(10);
		editor_view.set_right_margin(10);
		editor_view.set_bottom_margin(10);
		editor_view.set_left_margin(10);
		editor_view.set_indent(10);
		editor_view.set_vexpand(true);
		editor_view.set_valign(gtk::Align::Fill);
		editor_view.set_hexpand(true);
		editor_view.set_halign(gtk::Align::Fill);
		editor_view.set_wrap_mode(gtk::WrapMode::WordChar);
		// editor_view.show_line_numbers(true)
		// editor_view.monospace(true)
		// editor_view.tab_width(4)

		Self {
			path: path.to_string(),
			buffer,
			editor_view
		}
	}

	pub fn add_buffer(&self, path: &str) -> sourceview5::Buffer {
		let buffer = sourceview5::Buffer::new(None);
		//buffer.set_highlight_syntax(true);
		let file = gio::File::for_path(&path);
		let file = sourceview5::File::builder().location(&file).build();
		let loader = sourceview5::FileLoader::new(&buffer, &file);
		let _path_clone = path.to_string();

		loader.load_async_with_callback(
			glib::Priority::default(),
			gio::Cancellable::NONE,
			move |_current_num_bytes, _total_num_bytes| {
				// println!(
				// 	"loading {:?}: {:?}",
				// 	path_clone,
				// 	(current_num_bytes as f32 / total_num_bytes as f32) * 100f32
				// );
			},
			|_res| {
				// println!("loaded {:?}", res);
			}
		);

		buffer
	}

	pub fn update_path(&mut self, path: &str) {
		self.path = path.to_string();
		let buffer = self.add_buffer(path);
		self.editor_view.set_buffer(Some(&buffer));
	}

	pub fn get_view(&self) -> &View {
		&self.editor_view
	}
}

pub fn build_ui(editor: Rc<RefCell<Editor>>) -> gtk::Box {
	let editor_panel = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.css_classes(["editor-panel"])
		.build();

	let editor_panel_label = gtk::Label::builder()
		.label("Bellbird Notes")
		.margin_top(5)
		.margin_start(5)
		.margin_bottom(5)
		.halign(gtk::Align::Start)
		.build();

	let scrollable_window = gtk::ScrolledWindow::builder()
		.child(editor.borrow_mut().get_view())
		.build();

	editor_panel.append(&editor_panel_label);
	editor_panel.append(&scrollable_window);

	editor_panel
}
