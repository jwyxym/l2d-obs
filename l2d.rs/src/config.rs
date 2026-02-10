
use serde_derive::Deserialize;
use serde::Serialize;
use basic_toml::{from_str, to_string};
use std::fs::read_to_string;
use walkdir::WalkDir;
#[derive(Deserialize, Clone, Serialize)]
pub struct Config {
	server: ServerInfo
}

#[derive(Deserialize, Clone, Serialize)]
pub struct ServerInfo {
	address: String,
	port: i16,
	model: String,
	position: i8,
	micro: bool
}
impl Config {
	pub fn from (path: &str) -> Config {
		if let Ok(file) = read_to_string(path) {
			from_str::<Config>(&file).unwrap_or(Config::new("127.0.0.1", 8080))
		} else {
			Config::new("127.0.0.1", 8080)
		}
	}
	pub fn new (address: &str, port: i16) -> Config {
		let model: String = WalkDir::new("vts")
			.into_iter()
			.filter_map(Result::ok)
			.filter(|e| e.path().is_file())
			.find(|e| {
				e.path()
					.file_name()
					.and_then(|n| n.to_str())
					.map(|name| name.ends_with(".model3.json"))
					.unwrap_or(false)
			})
			.map(|e| {
				e.path()
					.strip_prefix("vts")
					.unwrap_or(e.path())
					.to_string_lossy()
					.replace('\\', "/")
					.trim_start_matches('/')
					.to_string()
			}).unwrap_or(String::from(""));
		let info: ServerInfo = ServerInfo{
			address: String::from(address),
			port: port,
			model: model,
			position: 4,
			micro: false
		};
		Config {
			server: info
		}
	}
	pub fn address (&self) -> String {
		self.server.address.clone()
	}
	pub fn port (&self) -> i16 {
		self.server.port
	}
	pub fn model (&self) -> String {
		self.server.model.clone()
	}
	pub fn position (&self) -> i8 {
		self.server.position
	}
	pub fn micro (&self) -> bool {
		self.server.micro
	}
	pub fn to_string (&self) -> String {
		to_string(&self).unwrap_or(String::from("err"))
	}
}