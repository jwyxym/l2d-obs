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

#[get("/index")]
pub async fn index() -> impl Responder {
	let html_content: &str = include_str!("../page/index.html");
	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(html_content)
}
#[get("/{js}")]
pub async fn js(req: HttpRequest) -> Result<impl Responder, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let js: &str = req.query("js");
	let content: &str = match js {
		"live2d.min.js" => include_str!("../page/live2d.min.js"),
		"live2dcubismcore.min.js" => include_str!("../page/live2dcubismcore.min.js"),
		_ => return Err(ErrorNotFound(""))
	};
	Ok(HttpResponse::Ok()
		.content_type("application/javascript; charset=utf-8")
		.body(content))
}