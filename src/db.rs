use sqlite;

use std::collections::HashMap;
use std::path::PathBuf;

use serde_derive::Serialize;

/// Information about some database
#[derive(Hash, Eq, PartialEq, Debug, Serialize)]
pub struct DBInfo {
	pub alias: String,
	pub path: PathBuf,
}

/// Handles all connetion info
pub struct ConnectionControler {
	pub connections: HashMap<DBInfo, sqlite::Connection>,
}

impl ConnectionControler {
	pub fn init() -> Self {
		let mut c: ConnectionControler = ConnectionControler {
			connections: HashMap::new(),
		};
		c.connect("db-exp", "db.sqlite3");

		c
	}

	pub fn connect(&mut self, alias: &str, path: &str) {
		let c = sqlite::open(path).unwrap();
		let inf = DBInfo {
			alias: alias.to_string(),
			path: PathBuf::from(path),
		};

		self.connections.insert(inf, c);
	}
}
