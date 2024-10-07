use gtk::gdk::Rectangle;
use gtk::prelude::*;
use gtk::gio;

pub mod notes_list_context_menu;
pub mod directory_tree_context_menu;

#[derive(Debug, Clone)]
pub struct BbMenuSection<'a> {
	pub label: Option<&'a str>,
	pub items: Vec<BbMenuItem<'a>>,
}

#[derive(Debug, Clone)]
pub struct BbMenuItem<'a> {
	pub label: &'a str,
	pub action: &'a str,
}

pub struct ContextMenu<'a> {
	pub sections: Vec<BbMenuSection<'a>>,
	pub parent: Vec<gtk::ListView>,
	pub width: i32,
	popover: gtk::PopoverMenu,
}

impl<'a> ContextMenu<'a> {
	pub fn new(
		sections: Vec<BbMenuSection<'a>>,
		parent: Vec<gtk::ListView>,
		width: i32
	) -> Self {
		let context_menu = gio::Menu::new();

		Self {
			sections,
			parent,
			width,
			popover: gtk::PopoverMenu::from_model(Some(&context_menu))
		}
	}

	pub fn build<F: 'static>(&self, on_pressed: F)
	where
		F: Fn(gtk::Widget) + Clone
	{
		self.parent.iter().for_each(|parent| {
			let context_menu = self.menu_model();
			let popover = gtk::PopoverMenu::from_model(Some(&context_menu));
			popover.set_menu_model(Some(&context_menu));
			popover.set_parent(parent);
			popover.set_has_arrow(false);
			popover.set_size_request(self.width, 0);

			let gesture = self.gesture();
			let list_view_clone = parent.clone();

			let on_pressed_clone = on_pressed.clone();
			gesture.connect_pressed(move |gesture ,_n_press , x, y| {
				if gesture.current_button() == 3 {
					let position = Rectangle::new(x as i32, y as i32, 1, 1);
					let (width, _) = popover.size_request();
					popover.set_pointing_to(Some(&position));
					popover.set_offset(width / 2, 0);
					popover.popup();

					if let Some(list_row_item) = list_view_clone.pick(x, y, gtk::PickFlags::DEFAULT) {
						on_pressed_clone(list_row_item);
					}
				}
			});

			parent.add_controller(gesture);
		});
	}

	fn menu_model(&self) -> gio::Menu {
		let context_menu = gio::Menu::new();

		for section in self.sections.iter() {
			let menu_items = gio::Menu::new();
			for item in section.items.iter() {
				let action_name = format!("app.{}", item.action);
				menu_items.append(Some(&item.label), Some(&action_name));
			}
			context_menu.append_section(section.label, &menu_items);
		}

		context_menu
	}

	fn gesture(&self) -> gtk::GestureClick {
		let gesture = gtk::GestureClick::new();
		gesture.set_button(3);
		gesture
	}
}
