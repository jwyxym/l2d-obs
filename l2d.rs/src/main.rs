mod config;
use config::Config;
mod ws;
use ws::WebSocket;
mod api;
mod micro;
mod global;

use actix_cors::Cors;
use actix_web::{
	App, HttpServer, web
};
use std::{
	fs::{create_dir_all, exists, write},
	io::Result
};

#[actix_web::main]
async fn main() -> Result<()> {
	let config: Config = Config::from("config.toml");
	if let Ok(exist) = exists("config.toml") && !exist {
		let _ = write("config.toml", config.to_string());
	}
	println!("{}", config.to_string());
	let port: i16 = config.port();
	let address: String = config.address();
	let _ = create_dir_all("vts");
	if config.micro() {
		let _ = micro::init();
	}
	let address: String = format!("{}:{}", address, port);
	println!("服务器监听于 {}", address);
	println!("请在obs导入 http://{}", address);
	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(config.clone()))
			.wrap(
				Cors::default()
				.allow_any_origin()
				.allow_any_method()
				.allow_any_header()
				.expose_any_header()
				.max_age(3600)
			)
			.route("/ws", web::get().to(WebSocket::route))
			.service(api::index)
			.service(api::web)
			.service(api::file)
	})
	.bind(address)?
	.run()
	.await
}
