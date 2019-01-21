use crate::db::DBExecutor;
use crate::AppState;
use actix::{Handler, Message};
use actix_web::{Error, HttpResponse, State};
use askama::Template;
use rusqlite::{Connection, NO_PARAMS};

/// Dashboard specific information
/// THis store cards info
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct Dashboard {
	databases: Vec<String>,
}

pub enum DashboardQuerys { }

impl Message for DashboardQuerys {
	type Result = Result<Dashboard, Error>;
}

impl Handler<DashboardQuerys> for DBExecutor {
	type Result = Result<Dashboard, Error>;

	fn handle(&mut self, msg: DashboardQuerys, _: &mut Self::Context) -> Self::Result {
		let conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager> = self.0.get().unwrap();
		let r = conn.query_row("select 1", NO_PARAMS, |r| r.get(0)).unwrap();

		Ok(
			Dashboard {
				databases: vec![r]
			}
		)
	}
}

/// Show the dashboard
pub fn show_dashboard(state: State<AppState>) -> Result<HttpResponse, Error> {
	let result = state.db.query_row("select 'DataBase Name'", NO_PARAMS, |r| r.get(0)).unwrap();

	let s = Dashboard {
		databases: vec!["DataBase 1".to_string(), "DataBase 2".to_string()],
	}
	.render()
	.unwrap();
	Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
