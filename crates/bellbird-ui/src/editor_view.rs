use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::cell::{Cell, RefCell};
//use std::sync::{Arc, Mutex};
use std::time::Duration;

use bellbird_core::config::{Config, ConfigOptions};
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
	//file_finished_loading: Arc<Mutex<f32>>,
}

impl<'a> Editor {
	pub fn new(path: &'a Path) -> Self {
		let buffer = sourceview5::Buffer::new(None);
		// let editor_view = View::builder().build();

		let editor_view = View::with_buffer(&buffer);
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

		let editor_breadcrumb = Breadcrumb::new();
		let editor_clone = editor_view.clone();
		let controller = gtk::EventControllerKey::builder()
			.propagation_phase(gtk::PropagationPhase::Capture)
			.build();

		controller.connect_key_released(move |_, _keyval, _keycode, _state| {
			editor_clone
				.activate_action("app.editor-key-up", Some(&"".to_variant()))
				.expect("The action `editor-key-up` does not exist.");
		});
		editor_view.add_controller(controller.clone());

		//let file_finished_loading = Arc::new(Mutex::new(0.0));

		Self {
			path: path.to_path_buf(),
			buffer,
			editor_view,
			editor_breadcrumb,
			//file_finished_loading,
		}
	}

	pub fn add_buffer(&self, path: &PathBuf) -> sourceview5::Buffer {
		let buffer = sourceview5::Buffer::new(None);
		//buffer.set_highlight_syntax(true);
		let file = gio::File::for_path(path);
		let file = sourceview5::File::builder().location(&file).build();
		let loader = sourceview5::FileLoader::new(&buffer, &file);
		//let path_clone = path.display().to_string();

		//let file_finished_loading = Arc::clone(&self.file_finished_loading);

		//loader.load_async(glib::Priority::default(), gio::Cancellable::NONE, move |_| {});
		loader.load_async_with_callback(
			glib::Priority::default(),
			gio::Cancellable::NONE,
			move |_current_num_bytes, _total_num_bytes| {
				//let percentage = (current_num_bytes as f32 / total_num_bytes as f32) * 100f32;
				//println!(
				//	"loading {:?}: {:?}",
				//	path_clone,
				//	percentage
				//);
				//let mut finished = file_finished_loading.lock().unwrap();
				//*finished = percentage;
			},
			move |_res| {

				//println!("loaded {:?}", res);
				//let mut finished = file_finished_loading.lock().unwrap();
				//*finished = _res.unwrap();
				//println!("{finished}");
			}
		);

		//println!("{}", *file_finished_loading.lock().unwrap());

		buffer
	}

	pub async fn update_path(&mut self, path: PathBuf) {
		self.path = path.clone();
		let buffer = self.add_buffer(&path);
		// disable editor if no note is loaded to avoid
		// writing into nothing
		//if self.path.exists() {
		//	self.set_editor_editable(true);
		//}
		//else {
		//	self.set_editor_editable(false);
		//}

		self.editor_view.set_buffer(Some(&buffer));
		self.editor_breadcrumb = self.build_breadcrumb().await.clone();
		self.editor_view.queue_draw();
		self.place_cursor(&buffer).await;
		self.editor_view.grab_focus();

		let _ = self.write_caret_position_to_file(buffer);
	}

	fn caret_position(&self) -> Option<i32> {
		match Config::new().meta_info(
			&self.path.display().to_string(),
			ConfigOptions::CaretPosition
		) {
			Some(position) => position.parse().ok(),
			_ => None
		}
	}

	// @todo make this work.
	// I somehow need to call this method when the file has finished loading
	async fn place_cursor(&self, buffer: &Buffer) {
		if let Some(cursor_position ) = self.caret_position() {
			let start_iter = buffer.start_iter();
			let buffer_length = buffer.text(
				&start_iter,
				&buffer.end_iter(),
				false
			).len() as i32;
			let position = cursor_position.clamp(0, buffer_length);
			let mut iter = buffer .start_iter();
			iter.set_offset(position);
			//println!("{:?} {:?} {:?}", cursor_position, position, buffer_length);
			buffer.place_cursor(&iter);
		}
	}

	fn write_caret_position_to_file(&self, buffer: Buffer) -> anyhow::Result<()> {
		let typing_timeout: Rc<Cell<Option<glib::SourceId>>> = Rc::new(Cell::new(None));
		let config = Config::new();

		buffer.connect_cursor_position_notify({
			let typing_timeout = typing_timeout.clone();
			let path = self.path.clone();
			move |buffer| {
				if let Some(timeout_id) = typing_timeout.take() {
					timeout_id.remove();
				}

				let timeout_id = glib::timeout_add_local(Duration::from_millis(500), {
					let buffer = buffer.clone();
					let typing_timeout = typing_timeout.clone();
					let path = path.clone();
					let mut config = config.clone();
					move || {
						let buffer = buffer.clone();
						let _ = config.set_meta_value(
							&path.display().to_string(),
							ConfigOptions::CaretPosition,
							buffer.cursor_position().to_string()
						);
						typing_timeout.set(None);
						glib::ControlFlow::Break
					}
				});

				typing_timeout.set(Some(timeout_id));
			}
		});
		Ok(())
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

	async fn build_breadcrumb(&self) -> &Breadcrumb {
		self.editor_breadcrumb.build(&self.path).await;
		&self.editor_breadcrumb
	}

	pub fn write_note(&self) {
		let buffer_clone = self.editor_view.buffer();
		let buffer_start = buffer_clone.start_iter();
		let buffer_end = buffer_clone.end_iter();
		let _ = Notes::write_to_file(
			self.path.clone(),
			buffer_clone.text(&buffer_start, &buffer_end, false).to_string()
		);
	}
}

pub fn build_ui(editor: &Rc<RefCell<Editor>>) -> gtk::Box {
	let editor_panel = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.css_classes(["editor-panel"])
		.margin_top(3)
		.margin_bottom(3)
		.margin_end(4)
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
