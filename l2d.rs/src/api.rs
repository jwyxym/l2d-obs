mod model;
use model::Model;

use actix_web::{
	Responder,
	HttpResponse,
	HttpRequest,
	get,
	dev,
	error::ErrorNotFound,
	Result,
	Error,
	web::Path,
	error::ErrorBadRequest
};
use std::fs::{read_to_string, read};
use mime_guess::from_path;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/index.html"))
        .finish()
}

#[get("/{key}")]
pub async fn web(req: HttpRequest) -> Result<impl Responder, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let key: &str = req.query("key");
	if key.starts_with("index") || key == "" {
		return Ok(HttpResponse::Ok()
			.content_type("text/html; charset=utf-8")
			.body(include_str!("../../l2d.js/dist/index.html")));
	}
	let content: &str = match key {
		"live2d.min.js" => include_str!("../../l2d.js/dist/live2d.min.js"),
		"live2dcubismcore.min.js" => include_str!("../../l2d.js/dist/live2dcubismcore.min.js"),
		_ => return Err(ErrorNotFound(""))
	};
	Ok(HttpResponse::Ok()
		.content_type("application/javascript; charset=utf-8")
		.body(content))
}

#[get(r#"/vts/{key:.*}"#)]
pub async fn file(key: Path<String>) -> Result<impl Responder, Error> {
	let path: String = format!("./vts/{}", key);
	if key.ends_with(".json") && let Ok(content) = read_to_string(&path) {
		if key.ends_with(".model3.json") && let Ok(model) = Model::from_json(&content) {
			if let Ok(content) = model.to_string() {
				return Ok(HttpResponse::Ok()
					.content_type("application/json")
					.body(content))
			} else {
				return Err(ErrorBadRequest("模型文件格式错误"));
			}
		}
		Ok(HttpResponse::Ok()
			.content_type("application/json")
			.body(content))
	} else {
		if let Ok(content) = read(&path) {
			let mime_type = from_path(&path)
        		.first_or_octet_stream();
			Ok(HttpResponse::Ok()
				.content_type(mime_type.as_ref())
				.body(content))
		} else {
			Err(ErrorBadRequest("读取文件失败"))
		}
	}
}