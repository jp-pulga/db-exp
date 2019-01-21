#![feature(generic_associated_types)]

use actix::Addr;
use actix_web::fs::StaticFiles;
use actix_web::{http, middleware, server, App};
use env_logger;

mod db;
mod handlers;
use crate::handlers::*;

pub struct AppState {
	pub db: Addr<db::DBExecutor>,
}

fn main() {
	::std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();
	let sys = actix::System::new("db-explorer");

	let addr = db::initialize_db_exp_connection();

	server::new(move || {
		App::with_state(AppState { db: addr.clone() })
			.middleware(middleware::Logger::default())
			.handler(
				"/static",
				StaticFiles::new("static").unwrap().show_files_listing(),
			)
			.resource("/", |r| {
				r.method(http::Method::GET).with(dashboard::show_dashboard)
			})
	})
	.bind("127.0.0.1:8080")
	.expect("Could not bind to port 8080")
	.run();

	let _ = sys.run();
}
