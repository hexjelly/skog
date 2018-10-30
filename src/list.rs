// use warp::http::StatusCode;

use super::R2d2Pool;

pub fn list_statuses(_pool: R2d2Pool) -> impl warp::Reply {
	// let conn = pool.get().unwrap();
	// conn.execute("INSERT INTO foo (bar) VALUES (?)", &[&i]).unwrap();

	warp::reply::json(&[1, 2, 3])
}
