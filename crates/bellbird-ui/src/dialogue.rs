use gtk::{gdk, prelude::*};

#[derive(Debug, Clone)]
pub struct Dialogue<'a> {
	app: &'a adw::Application,
	window: gtk::Window,
}

impl<'a> Dialogue<'a> {
	pub fn new(app: &'a adw::Application) -> Self {
		Self {
			app,
			window: gtk::Window::new(),
		}
	}

	pub fn input<F: 'static, C>(&self, title: &str, label: &str, placeholder: &str, ok: F, cancel: C)
	where
		F: Fn(String) + 'static + Clone,
		C: Fn() + 'static + Copy
	{
		self.create_window(title, 350, 0);

		let window_box = gtk::Box::builder()
			.orientation(gtk::Orientation::Vertical)
			.margin_start(15)
			.margin_end(15)
			.margin_top(10)
			.margin_bottom(10)
			.build();

		let label = self.label(label);
		let input = gtk::Text::builder()
			.text(placeholder)
			.build();

		let ok_clone = ok.clone();
		let window_clone = self.window.clone();
		input.connect_activate(glib::clone!(
			#[weak] window_clone,
			move |input| {
				ok_clone(input.text().to_string());
				window_clone.close();
			}
		));

		let input_clone = input.clone();
		let button_box = self.button_box();

		button_box.append(&self.ok_button(move || {
			let note_name = input_clone.text();
			ok(note_name.to_string());
		}));
		button_box.append(&self.cancel_button(move || cancel()));

		window_box.append(&label);
		window_box.append(&input);
		window_box.append(&button_box);

		self.key_events(&window_box);

		self.window.set_child(Some(&window_box));
		self.window.present();
	}

	pub fn warning_yes_no<F: 'static, C>(&self, title: &str, label: &str, description: &str, ok: F, cancel: C)
	where
		F: Fn(),
		C: Fn() + 'static
	{
		self.create_window(title, 300, 0);

		let window_box = gtk::Box::builder()
			.orientation(gtk::Orientation::Vertical)
			.margin_start(15)
			.margin_end(15)
			.margin_top(10)
			.margin_bottom(10)
			.build();

		let label = self.label(label);
		let description = self.label(description);
		let button_box = self.button_box();

		button_box.append(&self.ok_button(move || ok()));
		button_box.append(&self.cancel_button(move || cancel()));

		window_box.append(&label);
		window_box.append(&description);
		window_box.append(&button_box);

		self.key_events(&window_box);

		self.window.set_child(Some(&window_box));
		self.window.present();
	}

	fn create_window(&self, title: &str, width: i32, height: i32) {
		if let Some(active_window) = self.app.active_window() {
			self.window.set_application(Some(self.app));
			self.window.set_title(Some(title));
			self.window.set_width_request(width);
			self.window.set_height_request(height);
			self.window.set_destroy_with_parent(true);
			self.window.set_modal(true);
			self.window.set_css_classes(&["dialogue"]);
			self.window.set_transient_for(Some(&active_window));
		}
	}

	fn label(&self, label: &str) -> gtk::Label {
		gtk::Label::builder()
			.label(label)
			.margin_bottom(10)
			.halign(gtk::Align::Start)
			.build()
	}

	fn button_box(&self) -> gtk::Box {
		let button_box = gtk::Box::builder()
			.orientation(gtk::Orientation::Horizontal)
			.spacing(12)
			.halign(gtk::Align::End)
			.margin_top(15)
			.build();

		button_box
	}

	fn ok_button<F>(&self, ok: F) -> gtk::Button
	where
		F: Fn() + 'static
	{
		let ok_button = gtk::Button::builder()
			.label("Ok")
			.width_request(80)
			.can_focus(true)
			.css_classes(["button"])
			.build();

		//let ok_image = gtk::Image::from_icon_name("dialog-apply");
		//ok_button.set_child(Some(&ok_image));

		let window_clone = self.window.clone();
		ok_button.connect_clicked(move |_button| {
			ok();
			window_clone.close();
		});
		ok_button
	}

	fn cancel_button<F>(&self, cancel: F) -> gtk::Button
	where
		F: Fn() + 'static
	{
		let cancel_button = gtk::Button::builder()
			.label("Cancel")
			.width_request(80)
			.css_classes(["button"])
			.build();

		let window_clone = self.window.clone();
		cancel_button.connect_clicked(move |_button| {
			cancel();
			window_clone.close();
		});

		cancel_button
	}

	fn key_events(&self, window_box: &gtk::Box) {
		let controller = gtk::EventControllerKey::builder()
			.propagation_phase(gtk::PropagationPhase::Capture)
			.build();

		let window_clone = self.window.clone();
		controller.connect_key_released(move |_, key, _, _| {
				match key {
					gdk::Key::Escape => window_clone.close(),
					_ => (),
				}
			}
		);

		window_box.add_controller(controller);
	}
}
