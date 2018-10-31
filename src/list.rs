// use warp::http::StatusCode;

use super::{R2d2Pool, Status};
use rusqlite::NO_PARAMS;

pub fn list_statuses(pool: R2d2Pool) -> impl warp::Reply {
	let conn = pool.get().unwrap();
	let mut stmt = conn
		.prepare("SELECT id, date, update_date, eta, close_date, title, message FROM statuses")
		.unwrap();
	let statuses = stmt
		.query_map(NO_PARAMS, |row| Status {
			// id: row.get(0),
			date: row.get(1),
			update_date: row.get(2),
			eta: row.get(3),
			close_date: row.get(4),
			title: row.get(5),
			message: row.get(6),
		}).unwrap();

	let result: Result<Vec<Status>, _> = statuses.collect();

	warp::reply::json(&result.unwrap())
}
