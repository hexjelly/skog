use warp::http::Response;

use super::R2d2Pool;

pub fn delete_status(id: i64, pool: R2d2Pool) -> Result<impl warp::Reply, warp::Rejection> {
	let conn = pool.get().unwrap();
	match conn.execute("DELETE FROM statuses WHERE id = ?", &[&id]) {
		Ok(rows) => {
			if rows == 0 {
				debug!("delete_status: id={} error=id does not exist", id);
				let res = Response::builder()
					.status(404)
					.body(format!("id={} does not exist", id))
					.unwrap();
				Ok(res)
			} else {
				let res = Response::builder()
					.status(204)
					.body(format!("id={} deleted succesfully", id))
					.unwrap();
				Ok(res)
			}
		}
		Err(e) => {
			error!("delete_status: id={} error={}", id, e);
			let res = Response::builder()
				.status(500)
				.body(format!("id={} error={}", id, e))
				.unwrap();
			Ok(res)
		}
	}
}
