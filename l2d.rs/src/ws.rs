mod msg;
use msg::{MSG, MSGDATA};
use crate::Config;

use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix::prelude::*;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, MouseState};

pub struct WebSocket {
	config: web::Data<Config>
}
impl WebSocket {
	pub async fn route (req: HttpRequest, stream: web::Payload, config: web::Data<Config>) -> Result<HttpResponse, Error> {
		ws::start (WebSocket { config: config }, &req, stream)
	}
}

impl actix::Actor for WebSocket {
	type Context = ws::WebsocketContext<Self>;
	fn started (&mut self, ctx: &mut Self::Context) {
		ctx.run_interval(Duration::from_millis(10), |_, ctx| {
			if let Ok(screen) = screen_info::DisplayInfo::all() {
				if let Some(screen) = screen.first() {
					let device_state: DeviceState = DeviceState::new();
					let mouse: MouseState = device_state.get_mouse();

					let screen_width: f64 = screen.width as f64;
					let screen_height: f64 = screen.height as f64;
					let mouse_x: f64 = mouse.coords.0 as f64;
					let mouse_y: f64 = mouse.coords.1 as f64;

					let text: String = MSG::new(1, 
							MSGDATA::Array([
								(mouse_x - screen_width) / screen_width,
								(screen_height / 2.0 - mouse_y) / screen_height
							])
						).to_string();
					ctx.text(text);
				}
			}
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
					0 => MSG::new(0, MSGDATA::Text(self.config.model())),
					1 => MSG::new(1, MSGDATA::Array([0.5, 0.5])),
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