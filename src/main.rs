use fltk::{
	app,
	button::Button, 
	enums::CallbackTrigger,
	frame::Frame,
	input::*,
	prelude::*, 
	window::Window
};

use tokio::{
	time::{
		Duration,
		interval,
	},
};

#[derive (Clone, Copy)]
enum Message {
	PopUp,
	Submit,
}

#[tokio::main]
async fn main () -> Result <(), Error> {
	let (fltk_tx, fltk_rx) = app::channel::<Message> ();
	
	let app = app::App::default ();
	let mut wind = Window::new (100, 100, 640, 480, "Annoying Journal");
	
	let mut gui = Gui::new (fltk_tx);
	
	wind.end ();
	wind.show ();
	
	tokio::spawn (async move {
		let mut i = interval (Duration::from_secs (3));
		i.set_missed_tick_behavior (tokio::time::MissedTickBehavior::Skip);
		
		loop {
			i.tick ().await;
			eprintln! ("3SAHNQ43 Popping up");
			fltk_tx.send (Message::PopUp);
		}
	});
	
	while app.wait () {
		let msg = match fltk_rx.recv () {
			Some (x) => x,
			None => continue,
		};
		
		match msg {
			Message::Submit => {
				wind.iconize ();
			},
			Message::PopUp => {
				wind.show ();
			},
		}
	}
	
	Ok (())
}

#[derive (thiserror::Error, Debug)]
enum Error {
	
}

struct Gui {
	but: Button,
}

impl Gui {
	fn new (fltk_tx: app::Sender <Message>) -> Self {
		let mut but = Button::new (640 - 100, 480 - 50, 100, 50, "Submit");
		but.set_trigger (CallbackTrigger::Release);
		but.emit (fltk_tx, Message::Submit);
		
		Self {
			but,
		}
	}
}
