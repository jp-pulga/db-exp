use actix::{Actor, Addr, SyncContext, SyncArbiter};
use num_cpus;
use r2d2;
use r2d2_sqlite;

pub struct DBExecutor(pub r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>);

impl Actor for DBExecutor {
	type Context = SyncContext<Self>;
}

pub fn initialize_db_exp_connection() -> Addr<DBExecutor> {
	let manager = r2d2_sqlite::SqliteConnectionManager::file("weather.db");
	let pool = r2d2::Pool::<r2d2_sqlite::SqliteConnectionManager>::new(manager).unwrap();

	SyncArbiter::start(num_cpus::get(), move || DBExecutor(pool.clone()))
}