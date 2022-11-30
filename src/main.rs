use std::sync::mpsc::{channel, Receiver, TryRecvError};

use eframe::egui;
use frames::{GenAppData, Drawable, Control};
use log::info;

pub mod frames;
pub mod utils;

pub const API_URL: &str = "http://srv04.elpi-tech.ru";

async fn request() -> String {
    let client = reqwest_wasm::Client::new();
    let url = reqwest_wasm::Url::parse("http://localhost:8080").unwrap();
    info!("req url: {:?}", url);
    let req = client.get(url.clone());
    match req.send().await {
        Ok(resp) => {
            resp.text().await.unwrap()
        },
        Err(e) => e.to_string()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("This app only for wasm32 arch!")
}

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("startup..");
    console_error_panic_hook::set_once();
    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();
    let web_options = eframe::WebOptions::default();
    info!("start egui_app..");
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|_| Box::new(App::default())),
    ).expect("failed to start eframe");
}

struct App {
    gen_data: GenAppData,
    frames: Vec<Box<dyn Drawable>>,
    control: Control
}

impl Default for App {
    fn default() -> Self {
        let mut gen_data = GenAppData {
            token: String::new(),
            url: String::from(API_URL),
            user: None
        };
        let auth = frames::auth::Auth::new("auth", &mut gen_data);
        Self {
            gen_data: gen_data,
            frames: vec![Box::new(auth)],
            control: Control::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Area::new("main").show(ctx, |ui| {
            if ui.button("button").clicked() {
                // let (tx, rx) = channel::<String>();
                // self.rx = Some(rx);
                // wasm_bindgen_futures::spawn_local(async move {
                //     let answ = request().await;
                //     info!("request answer: {}", answ);
                //     tx.send(answ).unwrap();
                // });
                info!("button pushed");
            }
            // self.rx = match self.rx.take() {
            //     Some(rx) => match rx.try_recv() {
            //         Ok(val) => {
            //             self.text = val;
            //             None
            //         },
            //         Err(TryRecvError::Empty) => Some(rx),
            //         _ => None
            //     },
            //     None => None
            // };
            // ui.label(&self.text);
        });
        self.control.update(ctx, &mut self.gen_data, &mut self.frames);
    }
}

