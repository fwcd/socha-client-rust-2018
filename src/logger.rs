const LEVEL_NONE: i32 = 100;
const LEVEL_ERROR: i32 = 2;
const LEVEL_WARN: i32 = 1;
const LEVEL_INFO: i32 = 0;
const LEVEL_DEBUG: i32 = -1;
const LEVEL_TRACE: i32 = -2;
const LEVEL_DEEP_TRACE: i32 = -3;
const LEVEL_VERY_DEEP_TRACE: i32 = -4;

pub struct Logger {
	level: i32
}

impl Logger {
	fn log<F>(&self, prefix: &str, msg: F, level: i32) where F: Fn() -> String {
		if level <= self.level {
			println!("{}{}", prefix, msg());
		}
	}

	pub fn error<F>(&self, msg: F) where F: Fn() -> String           { self.log("[ERROR]       ", msg, LEVEL_ERROR); }

	pub fn warn<F>(&self, msg: F) where F: Fn() -> String            { self.log("[WARN]        ", msg, LEVEL_WARN); }

	pub fn info<F>(&self, msg: F) where F: Fn() -> String            { self.log("[INFO]        ", msg, LEVEL_INFO); }

	pub fn debug<F>(&self, msg: F) where F: Fn() -> String           { self.log("[DEBUG]       ", msg, LEVEL_DEBUG); }

	pub fn trace<F>(&self, msg: F) where F: Fn() -> String           { self.log("[TRACE]       ", msg, LEVEL_TRACE); }

	pub fn deep_trace<F>(&self, msg: F) where F: Fn() -> String      { self.log("[DEEP_TRACE]  ", msg, LEVEL_DEEP_TRACE); }

	pub fn very_deep_trace<F>(&self, msg: F) where F: Fn() -> String { self.log("[V_DEEP_TRACE]", msg, LEVEL_VERY_DEEP_TRACE); }
}

pub static LOG: Logger = Logger { level: LEVEL_TRACE };