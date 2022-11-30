use std::thread;
use std::time::Duration;

use eframe::egui;
use super::Control;
use super::Drawable;
use super::GenAppData;

pub struct Account {
	is_open: bool,
	name: String,
	frame2_data: String
}

impl Account {
	pub fn new(name: &str, spec_data: &str, gen_data: &GenAppData) -> Self {
		println!("[Account][OPEN] requst data for frame.. with token: {}", gen_data.token);
		thread::sleep(Duration::from_millis(500));
		println!("[Account][OPEN] parse and insert requested data.. complete!");
		Self {
			is_open: true,
			name: String::from(name),
			frame2_data: String::from(spec_data)
		}
	}
}

impl Drawable for Account {
	fn name<'a>(&'a self) -> &'a str {
		&self.name
	}

	fn redraw(&mut self, ctx: &egui::Context, gen_data: &mut GenAppData, ctl: &mut Control) {
		egui::Window::new(self.name())
		.open(&mut true)
		.show(ctx, |ui| {
			ui.heading("Hello World From Account!");
			ui.heading(format!("My specific data(String): {}", self.frame2_data));
		});
	}

	fn set_open(&mut self, st: bool) {
		self.is_open = st;
		println!("Account was open");
	}

	fn is_open(&self) -> bool {
		self.is_open
	}
}