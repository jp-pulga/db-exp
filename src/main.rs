use env_logger;
use tera::{compile_templates, Context};
use actix_web::{error, http, middleware, server, App, Error, HttpResponse, State};
use actix_web::fs::StaticFiles;

struct AppState {
	template: tera::Tera,
}

fn index(state: State<AppState>) -> Result<HttpResponse, Error> {
	let mut ctx = Context::new();
	ctx.insert("databases", &vec!["some", "some1", "some2", "some4"]);

	render_template(state, "index.html", &mut ctx)
}

fn render_template(state: State<AppState>, template: &str, context : &mut Context) -> Result<HttpResponse, Error> {
	let s = state
		.template
		.render(template, &context)
		.map_err(|_| error::ErrorInternalServerError("Template error"))?;
	Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn main() {
	::std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	server::new(|| {
		let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));

		App::with_state(AppState { template: tera })
			.middleware(middleware::Logger::default())
			.handler(
				"/static",
				StaticFiles::new("static").unwrap().show_files_listing(),
			)
			.resource("/", |r| r.method(http::Method::GET).with(index))
	})
	.bind("127.0.0.1:8080")
	.expect("Could not bind to port 8080")
	.run();
}
