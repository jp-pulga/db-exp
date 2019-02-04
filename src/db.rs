use actix::{Actor, Addr, SyncArbiter, SyncContext};
use num_cpus;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub struct DBExecutor(pub Pool<SqliteConnectionManager>);

impl Actor for DBExecutor {
	type Context = SyncContext<Self>;
}

pub fn initialize_db_exp_connection() -> Addr<DBExecutor> {
	let manager = SqliteConnectionManager::file("db-exp.db");
	let pool = Pool::<SqliteConnectionManager>::new(manager).unwrap();

	SyncArbiter::start(num_cpus::get(), move || DBExecutor(pool.clone()))
}
