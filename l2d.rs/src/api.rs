use actix_web::{
	Responder,
	HttpResponse,
	HttpRequest,
	get,
	dev,
	error::ErrorNotFound,
	Result,
	Error
};

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