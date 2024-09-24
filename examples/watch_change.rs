use clipboard_rs::{
	Clipboard, ClipboardContext, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext,
};
use std::{collections::HashSet, thread, time::Duration};

struct Manager {
	ctx: ClipboardContext,
}

impl Manager {
	pub fn new() -> Self {
		let ctx = ClipboardContext::new().unwrap();
		Manager { ctx }
	}
}

impl ClipboardHandler for Manager {
	fn on_clipboard_change(&mut self) {

		// println!("Formats: {}", self.ctx.available_formats().unwrap().join(", "));

		match self.ctx.has_formats(Some(HashSet::from([String::from("TEXT"), String::from("image/webp")]))) {
			Ok(formats) => {
				print!("Clipboard - text: {}, rtf: {}, html: {}, image: {}, files: {}",
					formats.text, formats.rtf, formats.html, formats.image, formats.files
				);
				if let Some(other) = formats.other {
					for (name, value) in other {
						print!(", {}: {}", name, value);
					}
				}
				println!();
			},
			Err(err) => {
				println!("Clipboard format error: {}", err.to_string());
			}
		}
	}
}

fn main() {
	let manager = Manager::new();

	let mut watcher = ClipboardWatcherContext::new().unwrap();

	let watcher_shutdown: clipboard_rs::WatcherShutdown =
		watcher.add_handler(manager).get_shutdown_channel();

	thread::spawn(move || {
		thread::sleep(Duration::from_secs(60));
		println!("stop watch!");
		watcher_shutdown.stop();
	});

	println!("start watch!");
	watcher.start_watch();
}
