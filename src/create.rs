use rusqlite::types::ToSql;
use warp::http::StatusCode;

use super::{R2d2Pool, Status};

pub fn create_status(status: Status, pool: R2d2Pool) -> Result<impl warp::Reply, warp::Rejection> {
	debug!("create_status: {:?}", status);
	let conn = pool.get().unwrap();
	conn
		.execute(
			"INSERT INTO statuses (date, update_date, eta, close_date, title, message)
       VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
			&[
				&status.date,
				&status.update_date as &ToSql,
				&status.eta,
				&status.close_date,
				&status.title,
				&status.message,
			],
		).unwrap();

	Ok(StatusCode::CREATED)
}
