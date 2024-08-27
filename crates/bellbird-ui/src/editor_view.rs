use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::cell::RefCell;

use bellbird_core::notes::Notes;
use gtk::{gio, glib, prelude::*};
use sourceview5::{
	self,
	Buffer,
	View,
	prelude::ViewExt
};

use crate::breadcrumb::Breadcrumb;

#[derive(Debug, Clone)]
pub struct Editor {
	pub path: PathBuf,
	pub buffer: Buffer,
	pub editor_view: View,
	pub editor_breadcrumb: Breadcrumb,
}

impl<'a> Editor	{
	pub fn new(path: &'a Path) -> Self {
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
		editor_view.set_highlight_current_line(true);
		//editor_view.set_show_line_numbers(true);
		//editor_view.set_monospace(true);
		//editor_view.set_tab_width(4);

		//let config = Config::new();
		//let editor_breadcrumb = gtk::Label::builder()
		//	.label(config.app_name())
		//	.margin_top(5)
		//	.margin_start(5)
		//	.margin_bottom(5)
		//	.halign(gtk::Align::Start)
		//	.build();
		let editor_breadcrumb = Breadcrumb::new();

		Self {
			path: path.to_path_buf(),
			buffer,
			editor_view,
			editor_breadcrumb
		}
	}

	pub fn add_buffer(&self, path: PathBuf) -> sourceview5::Buffer {
		let buffer = sourceview5::Buffer::new(None);
		//buffer.set_highlight_syntax(true);
		let file = gio::File::for_path(path);
		let file = sourceview5::File::builder().location(&file).build();
		let loader = sourceview5::FileLoader::new(&buffer, &file);
		//let _path_clone = path.to_string();

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

	pub fn update_path(&mut self, path: PathBuf) {
		// @todo: all this cloning makes me sick...
		// try to handle this with lifetimes
		self.path = path.clone();
		let buffer = self.add_buffer(path.clone());
		self.editor_view.set_buffer(Some(&buffer));
		//self.editor_breadcrumb.set_label(&self.build_breadcrumb());
		self.editor_breadcrumb = self.build_breadcrumb().clone();

		// disable editor if no note is loaded to avoid
		// writing into nothing
		let controller = gtk::EventControllerKey::new();
		if self.path.exists() {
			let buffer_clone = buffer.clone();
			let path_clone = path.clone();
			controller.connect_key_released(move |_, _keyval, _keycode, _state| {
				let buffer_start = buffer_clone.start_iter();
				let buffer_end = buffer_clone.end_iter();
				Notes::write_to_file(
					&path_clone,
					buffer_clone.text(&buffer_start, &buffer_end, false).to_string()
				);
			});
			controller.set_propagation_phase(gtk::PropagationPhase::Capture);
			self.editor_view.add_controller(controller);
			self.set_editor_editable(true);
		}
		else {
			self.set_editor_editable(false);
		}
	}

	pub fn view(&self) -> &View {
		&self.editor_view
	}

	pub fn breadcrumb(&self) -> &Breadcrumb {
		&self.editor_breadcrumb
	}

	pub fn editor_editable(&self) -> bool {
		self.editor_view.is_editable() && self.editor_view.is_cursor_visible()
	}

	pub fn set_editor_editable(&self, editable: bool) {
		self.editor_view.set_editable(editable);
		self.editor_view.set_cursor_visible(editable);
	}

	fn build_breadcrumb(&self) -> &Breadcrumb {
		self.editor_breadcrumb.build(&self.path);
		&self.editor_breadcrumb
	}
}

pub fn build_ui(editor: Rc<RefCell<Editor>>) -> gtk::Box {
	let editor_panel = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.css_classes(["editor-panel"])
		.build();

	let editor_top_bar = gtk::Box::builder()
		.orientation(gtk::Orientation::Horizontal)
		.build();

	editor_top_bar.append(editor.borrow_mut().breadcrumb());

	let scrollable_window = gtk::ScrolledWindow::builder()
		.child(editor.borrow_mut().view())
		.build();

	let handle_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
	handle_box.append(&gtk::WindowControls::new(gtk::PackType::End));
	let _window_handle = gtk::WindowHandle::builder()
		.child(&handle_box)
		.build();

	//editor_panel.append(&_window_handle);
	editor_panel.append(&editor_top_bar);
	editor_panel.append(&scrollable_window);

	editor_panel
}
