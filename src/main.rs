use std::{
	str::FromStr,
};

use chrono::{
	Local,
	SecondsFormat,
};

use fltk::{
	app,
	button,
	prelude::*,
	text,
	window::Window,
};

use tokio::{
	fs,
	io::AsyncWriteExt,
	time::{
		Duration,
		interval,
	},
};

#[tokio::main]
async fn main () -> Result <(), Error> {
	let mut config = Config::default ();
	
	let mut args = std::env::args ();
	
	// Finally found the difference: https://stackoverflow.com/questions/1788923/parameter-vs-argument
	
	while let Some (arg) = args.next () {
		if arg == "--interval-secs" {
			let val = args.next ().ok_or (Error::ParamNeedsArg ("--interval-secs"))?;
			let val = u64::from_str (&val).map_err (|_| Error::CannotParseArg ("--interval-secs <u64>"))?;
			config.interval_secs = val;
		}
		else if arg == "--prompt" {
			let val = args.next ().ok_or (Error::ParamNeedsArg ("--prompt"))?;
			config.prompt = val;
		}
	}
	
	let (fltk_tx, fltk_rx) = app::channel::<Message> ();
	
	let app = app::App::default ();
	
	tokio::spawn (async move {
		let mut i = interval (Duration::from_secs (config.interval_secs));
		i.set_missed_tick_behavior (tokio::time::MissedTickBehavior::Skip);
		
		loop {
			i.tick ().await;
			eprintln! ("3SAHNQ43 Popping up");
			fltk_tx.send (Message::PopUp);
		}
	});
	
	let mut gui = Gui::new (config, fltk_tx);
	
	while app.wait () {
		let msg = match fltk_rx.recv () {
			Some (x) => x,
			None => continue,
		};
		
		match msg {
			Message::Submit => {
				if let Err (e) = gui.submit ().await {
					eprintln! ("DVW4SBNB Error while submitting: {:?}", e);
				}
			},
			Message::PopUp => {
				if let Err (e) = gui.pop_up () {
					eprintln! ("5BWNNQT6 Error while popping up: {:?}", e);
				}
			},
		}
	}
	
	Ok (())
}

struct Config {
	interval_secs: u64,
	prompt: String,
}

impl Default for Config {
	fn default () -> Self {
		Self {
			interval_secs: 2225,
			prompt: "Write a journal entry, then hit Tab, Enter to submit it.".into (),
		}
	}
}

#[derive (Clone, Copy)]
enum Message {
	PopUp,
	Submit,
}

#[derive (thiserror::Error, Debug)]
enum Error {
	#[error ("46MVLSEL Cannot parse argument: {0}")]
	CannotParseArg (&'static str),
	#[error ("4JZ5B2FN Editor has no buffer, this should be impossible")]
	EditorHasNoBuffer,
	#[error ("OKE7Z5O6 FLTK: {0}")]
	Fltk (#[from] FltkError),
	#[error ("4BQPBIAJ IO")]
	Io (#[from] std::io::Error),
	#[error ("KDP4DNOP JSON serialization failed")]
	JsonSerialization (#[from] serde_json::Error),
	#[error ("3MYHBQWV Parameter {0} needs an argument")]
	ParamNeedsArg (&'static str),
}

#[derive (serde::Serialize)]
struct JournalLine {
	text: String,
	time_submitted: String,
}

struct Gui {
	armed: bool,
	editor: text::TextEditor,
	wind: Window,
}

impl Gui {
	fn new (config: Config, fltk_tx: app::Sender <Message>) -> Self {
		let mut wind = Window::new (100, 100, 640, 480, "Annoying Journal");
		wind.make_resizable (true);
		
		let mut buffer = text::TextBuffer::default ();
		buffer.set_text (&config.prompt);
		let mut display = text::TextDisplay::new (0, 0, 640, 50, "");
		display.set_buffer (Some (buffer));
		
		let mut editor = text::TextEditor::new (0, 50, 640, 480 - 50 - 50, "");
		editor.set_buffer (Some (text::TextBuffer::default ()));
		
		editor.set_tab_nav (true);
		
		let mut but = button::ReturnButton::new (640 - 100, 480 - 50, 100, 50, "Submit");
		but.emit (fltk_tx, Message::Submit);
		
		wind.end ();
		wind.show ();
		
		Self {
			armed: true,
			editor,
			wind,
		}
	}
	
	fn pop_up (&mut self) -> Result <(), Error> {
		if ! self.armed {
			eprintln! ("O4U6E36V Ignoring pop-up, not armed");
			return Ok (());
		}
		
		self.armed = false;
		self.wind.show ();
		self.editor.take_focus ()?;
		
		Ok (())
	}
	
	async fn submit (&mut self) -> Result <(), Error> {
		let buffer = match self.editor.buffer () {
			None => return Err (Error::EditorHasNoBuffer),
			Some (x) => x,
		};
		
		let jl = JournalLine {
			text: buffer.text (),
			time_submitted: Local::now ().to_rfc3339_opts (SecondsFormat::Secs, true),
		};
		
		let s = serde_json::to_string (&jl)?;
		
		fs::create_dir_all ("annoying_journal").await?;
		
		let mut f = fs::OpenOptions::new ()
		.append (true)
		.create (true)
		.open ("annoying_journal/journal.jsonl").await?;
		f.write_all (s.as_bytes ()).await?;
		f.write_all (b"\n").await?;
		
		println! ("{}", s);
		self.editor.set_buffer (text::TextBuffer::default ());
		self.wind.iconize ();
		self.armed = true;
		
		Ok (())
	}
}
