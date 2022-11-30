use eframe::egui;
use super::Control;
use super::Drawable;
use super::GenAppData;

pub struct Navig {
	is_open: bool,
	name: String
}

impl Navig {
	pub fn new(name: &str) -> Self {
		Self {
			is_open: true,
			name: String::from(name)
		}
	}
}

impl Drawable for Navig {
	fn name<'a>(&'a self) -> &'a str {
		&self.name
	}

	fn redraw(&mut self, ctx: &egui::Context, _gen_data: &mut GenAppData, ctl: &mut Control) {
		egui::Window::new(self.name())
		.open(&mut true)
		.show(ctx, |ui| {
			ui.heading("Hello World From Navig!");
		});
	}

	fn set_open(&mut self, st: bool) {
		self.is_open = st;
		println!("Navig was open");
	}

	fn is_open(&self) -> bool {
		self.is_open
	}
}