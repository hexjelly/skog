use rusqlite::types::ToSql;
use warp::http::StatusCode;

use super::{R2d2Pool, Status};

pub fn update_status(
	id: i64,
	status: Status,
	pool: R2d2Pool,
) -> Result<impl warp::Reply, warp::Rejection> {
	debug!("update_status: {:?}", status);
	let conn = pool.get().unwrap();
	conn
		.execute(
			"UPDATE statuses
			 SET date = ?2, update_date = ?3, eta = ?4, close_date = ?5, title = ?6, message = ?7)
			 WHERE id = ?1",
			&[
				&id,
				&status.date as &ToSql,
				&status.update_date,
				&status.eta,
				&status.close_date,
				&status.title,
				&status.message,
			],
		).unwrap();

	Ok(StatusCode::CREATED)
}
