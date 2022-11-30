use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use eframe::egui;
use egui::Align2;
use egui::Color32;
use egui::FontId;
use egui::RichText;
use reqwest_wasm::Method;
use reqwest_wasm::header::AUTHORIZATION;
use serde::Deserialize;
use super::Control;
use super::Drawable;
use super::GenAppData;
use crate::utils::req::request;
use crate::utils::req::try_get;

#[derive(Deserialize)]
pub struct User {
	pub name: String,
	pub last_name: String,
	pub email: String
}

pub struct Auth {
	is_open: bool,
	name: String,
	login: String,
	passw: String,
	req: Option<Receiver<Result<Vec<u8>, String>>>,
	err_msg: String
}

impl Auth {
	pub fn new(name: &str, _gen_data: &mut GenAppData) -> Self {
		Self {
			is_open: true,
			name: String::from(name),
			req: None,
			login: format!("icupken@ya.ru"),
			passw: String::new(),
			err_msg: String::new()
		}
	}
}

impl Drawable for Auth {
	fn name<'a>(&'a self) -> &'a str {
		&self.name
	}

	fn redraw(&mut self, ctx: &egui::Context, data: &mut GenAppData, ctl: &mut Control) {
        egui::Window::new(self.name())
		.collapsible(false)
		.resizable(false)
		.title_bar(false)
		//.open(&mut false)
		.anchor(Align2::CENTER_CENTER, [0.0, 0.0])
		.open(&mut true)
		.show(ctx, |ui| {
			ui.vertical_centered(|ui| {
				ui.label(RichText::new("Логин").font(FontId::proportional(26.0)));
				ui.add(egui::TextEdit::singleline(&mut self.login));
				ui.label(RichText::new("Пароль").font(FontId::proportional(26.0)));
				ui.add(egui::TextEdit::singleline(&mut self.passw).password(true));
				if !self.err_msg.is_empty() {
					ui.label(RichText::new(&self.err_msg).color(Color32::DARK_RED));
				}
				if ui.button("Авторизоваться").clicked() {
					// if request not already proceeded
					if self.req.is_none() {
						let str_for_hash = format!("{}:{}", self.login, self.passw);
						data.token = sha256::digest(str_for_hash);
						let client = reqwest_wasm::Client::new();
						let req = client.request(Method::GET, &data.url).header(AUTHORIZATION, &data.token).build().unwrap();
						self.req = Some(request(req));
					}
				}

				// if request proceeded
				if self.req.is_some() {
					let req = self.req.take().unwrap();
					self.req = match try_get(&req) {
						// request completed, handle result
						Some(res) => match res {
							// request success
							Ok(answ_raw) => {
								// handle asnwer
								match rmp_serde::from_slice(&answ_raw) {
									Ok(answ) => {
										data.user = Some(answ);
										ctl.delete_frame(self.name());
										ctl.new_frame(Box::new(super::navig::Navig::new("navig")));
									},
									Err(e) => self.err_msg = e.to_string()
								}
								// return None(doesn't metter what the handling result) because request was taking, nothing for handle
								None
							},
							// request failed, show error text
							Err(e) => {
								self.err_msg = e;
								// return None because request was taking, nothing for handle
								None
							}
						},
						// return request handle for handle it in next iteration
						None => Some(req)
					}
				}
			});
		});
	}

	fn set_open(&mut self, st: bool) {
		self.is_open = st;
		println!("Auth is_open: {}", st);
	}

	fn is_open(&self) -> bool {
		self.is_open
	}
}