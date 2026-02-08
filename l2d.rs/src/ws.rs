mod msg;
use msg::{MSG, MSGDATA};
use crate::Config;

use actix_web::{web, Error, HttpRequest, HttpResponse,error::ErrorBadRequest};
use actix_web_actors::ws;
use actix::prelude::*;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, MouseState};

pub struct WebSocket {
	config: web::Data<Config>,
	height: f64,
	width: f64
}
impl WebSocket {
	pub async fn route (req: HttpRequest, stream: web::Payload, config: web::Data<Config>) -> Result<HttpResponse, Error> {
		let position: i8 = config.position();
		if position > -1 && position < 5 {
			if let Ok(screen) = screen_info::DisplayInfo::all() {
				if let Some(screen) = screen.first() {
					return ws::start (WebSocket {
							config: config,
							height: screen.height as f64,
							width: screen.width as f64
						},
						&req, stream
					);
				}
			}
			return Err(ErrorBadRequest("cant find screen"));
		}
		Err(ErrorBadRequest("config error : position"))
	}
}

impl actix::Actor for WebSocket {
	type Context = ws::WebsocketContext<Self>;
	fn started (&mut self, ctx: &mut Self::Context) {
		let screen_width: f64 = self.width;
		let screen_height: f64 = self.height;
		let position: i8 = self.config.position();
		ctx.run_interval(Duration::from_millis(10), move |_, ctx| {
			let device_state: DeviceState = DeviceState::new();
			let mouse: MouseState = device_state.get_mouse();
			let mouse_x: f64 = mouse.coords.0 as f64;
			let mouse_y: f64 = mouse.coords.1 as f64;
			let msg: MSG = match position {
				0..=4 => {
					let x: f64 = (mouse_x - screen_width / 2.0) / screen_width;
					let y: f64 = (screen_height / 2.0 - mouse_y) / screen_height;
					
					let (x, y) = match position {
						0 => (x, y),
						1 => (x + 0.5, -y),
						2 => (x - 0.5, -y),
						3 => (x + 0.5, y),
						4 => (x - 0.5, y),
						_ => unreachable!()
					};
					
					MSG::new(2, MSGDATA::Array([x, y]))
				}
				_ => MSG::new(-1, MSGDATA::Number(-1))
			};
			ctx.text(msg.to_string())
		});
	}
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
	fn handle (&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
		match msg {
			Ok(ws::Message::Ping(msg)) => {
				ctx.pong(&msg);
			}
			Ok(ws::Message::Text(text)) => {
				let msg: MSG = MSG::from_json(text.to_string());
				let text: String = match msg.protocol() {
					0 => MSG::new(0, MSGDATA::Number(self.config.position())),
					1 => MSG::new(1, MSGDATA::Text(self.config.model())),
					2 => MSG::new(2, MSGDATA::Array([0.5, 0.5])),
					_ => MSG::new(-1, MSGDATA::Number(-1))
				}.to_string();
				ctx.text(text);
			}
			Ok(ws::Message::Close(reason)) => {
				ctx.close(reason);
				ctx.stop();
			}
			_ => {}
		}
	}
}