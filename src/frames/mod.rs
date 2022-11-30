use self::auth::User;

pub mod auth;
pub mod account;
pub mod navig;

/// Contains common fields for all frames and App state
pub struct GenAppData {
	pub token: String,
	pub url: String,
	pub user: Option<User>
	
	// And others
}

pub enum ControlRequest {
	Close(String),				// FrameName
	Open(String),				// FrameName
	OpenExact(String),			// FrameName
	New(Box<dyn Drawable>),		// Frame
	Delete(String)				// FrameName
}

pub struct Control {
	requests: Vec<ControlRequest>
}

impl Default for Control {
	fn default() -> Self {
		Self {
			requests: Vec::new()
		}
	}
}

impl Control {
	/// For top level handler
	pub fn update(&mut self, ctx: &egui::Context, gen_data: &mut GenAppData, frames: &mut Vec<Box<dyn Drawable>>) {
		loop {
			if self.requests.len() > 0 {
				let req = self.requests.remove(0);
				match req {
					ControlRequest::Close(name) => {
						frames.iter_mut().for_each(|f| {
							if f.name() == name {
								f.set_open(false);
							}
						});
					},
					ControlRequest::Open(name) => {
						frames.iter_mut().for_each(|f| {
							if f.name() == name {
								f.set_open(true);
							}
						});
					},
					ControlRequest::OpenExact(name) => {
						frames.iter_mut().for_each(|f| {
							if f.name() == name {
								f.set_open(true);
							} else {
								f.set_open(false);
							}
						});
					},
					ControlRequest::New(frame) => {
						frames.push(frame);
					},
					ControlRequest::Delete(name) => {
						for i in 0..frames.len() {
							if frames[i].name() == name {
								frames.remove(i);
								break;
							}
						}
					}
				}
			} else {
				break;
			}
		}

		for f in frames {
			f.redraw(ctx, gen_data, self);
		}
	}

	/// Hide window, but don't delete frame data, frame with same data may be shown again with call 'open' func 
	pub fn close(&mut self, frame: &str) {
		self.requests.push(ControlRequest::Close(String::from(frame)));
	}

	/// Open specefied by name window, and close others
	pub fn open_exact(&mut self, frame: &str) {
		self.requests.push(ControlRequest::OpenExact(String::from(frame)));
	}

	/// Open specefied by name window, do not touch others
	pub fn open(&mut self, frame: &str) {
		self.requests.push(ControlRequest::Open(String::from(frame)));
	}

	/// Add new window in redraw loop, if frame with the same name was exists - it will be deleted
	pub fn new_frame(&mut self, frame: Box<dyn Drawable>) {
		self.delete_frame(frame.name());
		self.requests.push(ControlRequest::New(frame));
	}

	/// Delete frame from redraw loop, frame data will be non recoverable
	pub fn delete_frame(&mut self, name: &str) {
		self.requests.push(ControlRequest::Delete(String::from(name)));
	}
}

pub trait Drawable {
	/// `&'a` so we can also use it as a key to store open/close state.
	fn name<'a>(&'a self) -> &'a str;

	/// Show window, etc
	fn redraw(&mut self, ctx: &egui::Context, gen_data: &mut GenAppData, ctl: &mut Control);

	/// Set showable, do not for initialize, may use for reload content
	/// Initialize window (setting necessary  params for view and load data) performs while call custom for frame 'new' or building struct
	fn set_open(&mut self, is_open: bool);

	/// Return showable state
	fn is_open(&self) -> bool;
}