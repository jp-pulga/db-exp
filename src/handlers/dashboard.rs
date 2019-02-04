use crate::db::DBExecutor;
use crate::AppState;
use actix::{Handler, Message};
use actix_web::{AsyncResponder, Error, HttpResponse, State};
use askama::Template;
use futures::future::Future;
use rusqlite::NO_PARAMS;

/// Dashboard specific information
/// THis store cards info
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct Dashboard {
	databases: Vec<String>,
}

struct DashboardQuerys {}

impl Message for DashboardQuerys {
	type Result = Result<Dashboard, Error>;
}

impl Handler<DashboardQuerys> for DBExecutor {
	type Result = Result<Dashboard, Error>;

	fn handle(&mut self, _msg: DashboardQuerys, _: &mut Self::Context) -> Self::Result {
		let conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager> =
			self.0.get().unwrap();
		let result = conn
			.query_row("select name from databases", NO_PARAMS, |r| r.get(0))
			.unwrap();

		println!("{}", result);

		Ok(Dashboard {
			databases: vec![result],
		})
	}
}

/// Show the dashboard
pub fn show_dashboard(state: State<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
	state
		.db
		.send(DashboardQuerys {})
		.from_err()
		.and_then(|res| match res {
			Ok(v) => Ok(HttpResponse::Ok()
				.content_type("text/html")
				.body(v.render().unwrap())),
			Err(_) => Ok(HttpResponse::InternalServerError().into()),
		})
		.responder()
}
