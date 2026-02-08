use serde::{Deserialize, Serialize};
use serde_json::{self, Value, from_str, from_value, to_string};

#[derive(Debug, Serialize, Deserialize)]
pub enum MSGDATA {
	Number(i8),
	Text(String),
	Array([f64; 2])
}
#[derive(serde::Serialize)]
pub struct MSG {
	protocol: i8,
	data: MSGDATA
}

impl MSG {
	pub fn new (protocol: i8, data : MSGDATA) -> MSG {
		MSG {
			protocol,
			data,
		}
	}
	pub fn from_json(json: String) -> MSG {
		#[derive(Deserialize)]
		struct Message {
			protocol: i8,
			data: Value,
		}
		
		if let Ok(raw) = from_str::<Message>(&json) {
			let data: MSGDATA = match raw.protocol {
				0 => MSGDATA::Number(from_value(raw.data).unwrap_or(-1)),
				1 => MSGDATA::Text(from_value(raw.data).unwrap_or(String::from(""))),
				_ => MSGDATA::Text(String::from("")),
			};
			
			MSG {
				protocol: raw.protocol,
				data,
			}
		} else {
			MSG {
				protocol: -1,
				data: MSGDATA::Text(String::from("")),
			}
		}
	}
	pub fn protocol(&self) -> i8 {
		self.protocol
	}
	pub fn to_string(&self) -> String {
		to_string(&self).unwrap_or(to_string(&MSG::new(-1, MSGDATA::Number(-1))).unwrap())
	}
}