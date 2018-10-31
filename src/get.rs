use warp::http::Response;

use super::{R2d2Pool, Status};

pub fn get_status(id: i64, pool: R2d2Pool) -> Result<impl warp::Reply, warp::Rejection> {
	let conn = pool.get().unwrap();
	let mut stmt = conn
		.prepare(
			"SELECT id, date, update_date, eta, close_date, title, message
							FROM statuses
							WHERE id = ?",
		).unwrap();
	let query_res = stmt.query_row(&[&id], |row| Status {
		id: row.get(0),
		date: row.get(1),
		update_date: row.get(2),
		eta: row.get(3),
		close_date: row.get(4),
		title: row.get(5),
		message: row.get(6),
	});

	match query_res {
		Ok(status) => {
			let res = Response::builder()
				.status(200)
				.body(serde_json::to_string(&status).unwrap())
				.unwrap();
			Ok(res)
		}
		Err(e) => match e {
			rusqlite::Error::QueryReturnedNoRows => {
				let res = Response::builder()
					.status(404)
					.body("No row with that id".into())
					.unwrap();
				Ok(res)
			}
			_ => {
				error!("get_status: id={} error={}", id, e);
				Err(warp::reject::custom(e))
			}
		},
	}
}
