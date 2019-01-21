use crate::db::DBExecutor;
use crate::AppState;
use actix::{Handler, Message};
use actix_web::{Error, HttpResponse, State};
use askama::Template;

/// Dashboard specific information
/// THis store cards info
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct Dashboard<'a> {
	databases: Vec<&'a str>,
}

pub enum DashboardQuerys { }

impl Message for DashboardQuerys {
	type Result<'a> = Result<Vec<Dashboard<'a>>, Error>;
}

impl Handler<DashboardQuerys> for DBExecutor {
	type Result<'a> = Result<Vec<Dashboard<'a>>, Error>;

	fn handle(&mut self, msg: DashboardQuerys, _: &mut Self::Context) -> Self::Result {
		let conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager> = self.0.get()?;
		let r = conn.query_row("select 1").unwrap();

	}
}

/// Show the dashboard
pub fn show_dashboard(state: State<AppState>) -> Result<HttpResponse, Error> {
	let result = state.db.query_row("select 'DataBase Name'").unwrap();

	let s = Dashboard {
		databases: vec!["DataBase 1", "DataBase 2"],
	}
	.render()
	.unwrap();
	Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
