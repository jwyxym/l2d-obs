mod config;
use config::Config;
mod ws;
use ws::WebSocket;
mod api;

use actix_files::Files;

use actix_cors::Cors;
use actix_web::{
	HttpServer,
	App,
	web
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
	let port: i16 = config.port();
	let _ = create_dir_all("vts");
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
			.service(Files::new("/vts", "./vts")
				.show_files_listing()
				.use_last_modified(true)
				.use_etag(true)
				.prefer_utf8(true)
				.disable_content_disposition()
			)
			.service(api::index)
			.service(api::web)
	})
	.bind(format!("127.0.0.1:{}", port))?
	.run()
	.await
}
