use log::{Record, Level, Metadata};

struct LogChamp;

impl log::Log for LogChamp {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= Level::Trace
	}

	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			let color = match record.level() {
				Level::Error => "\x1b[91m",
				Level::Warn => 	"\x1b[33m",
				Level::Info => 	"\x1b[0m",
				Level::Debug => "\x1b[36m",
				Level::Trace => "\x1b[90m",
			};
			match record.level() {
				Level::Error | Level::Warn => {
					eprintln!("{color}[{0}]: {1}\x1b[0m", record.level(), record.args());
				}
				Level::Info | Level::Debug | Level::Trace => {
					println!("{color}[{0}]: {1}\x1b[0m", record.level(), record.args());
				}
			}
		}
	}

	fn flush(&self) {}
}

use log::{SetLoggerError, LevelFilter};

static LOGGER: LogChamp = LogChamp;

pub fn init() -> Result<(), SetLoggerError> {
	log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}