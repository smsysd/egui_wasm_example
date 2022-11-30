use std::sync::mpsc::{Receiver, channel, TryRecvError};
use reqwest_wasm::{Request, StatusCode};
use log::info;

pub fn try_get(req: &Receiver<Result<Vec<u8>, String>>) -> Option<Result<Vec<u8>, String>> {
	match req.try_recv() {
		Ok(res) => Some(res),
		Err(TryRecvError::Empty) => None,
		Err(TryRecvError::Disconnected) => None
	}
}

pub fn request(req: Request) -> Receiver<Result<Vec<u8>, String>> {
	let (tx, rx) = channel::<Result<Vec<u8>, String>>();
	wasm_bindgen_futures::spawn_local(async move {
		let client = reqwest_wasm::Client::new();
		match client.execute(req).await {
			Ok(resp) => {
				match resp.status() {
					StatusCode::OK => {
						match resp.bytes().await {
							Ok(raw) => {
								info!("request success received: {} bytes", raw.len());
								tx.send(Ok(raw.to_vec())).unwrap();
							},
							Err(e) => {
								info!("fail to get body: {:?}", e);
								tx.send(Err(format!("fail to get body: {:?}", e))).unwrap();
							}
						}
					},
					oth => {
						let text = match resp.text().await {
							Ok(text) => text,
							Err(_) => String::new()
						};
						info!("server return not 'OK' code: {} with text: {}", oth, text);
						tx.send(Err(format!("server return not 'OK' code: {} with text: {}", oth, text))).unwrap();
					}
				}
			},
			Err(e) => {
				info!("fail to execute request: {:?}", e);
				tx.send(Err(format!("Fail request to server: server unreachable or blocked"))).unwrap();
			}
		}
	});
	rx
}