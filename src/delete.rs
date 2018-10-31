use warp::http::StatusCode;

use super::R2d2Pool;

pub fn delete_status(id: i64, pool: R2d2Pool) -> Result<impl warp::Reply, warp::Rejection> {
	let conn = pool.get().unwrap();
	match conn.execute("DELETE FROM statuses WHERE id = ?", &[&id]) {
		Ok(rows) => {
			if rows == 0 {
				debug!("delete_todo: id={} error=Id does not exist", id);
				Ok(StatusCode::NOT_FOUND)
			} else {
				Ok(StatusCode::NO_CONTENT)
			}
		}
		Err(e) => {
			error!("delete_todo: id={} error={}", id, e);
			Err(warp::reject::not_found()) // TODO: better reject/error
		}
	}
}
