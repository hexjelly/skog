use warp::http::StatusCode;

use super::R2d2Pool;

pub fn delete_status(id: u64, _pool: R2d2Pool) -> Result<impl warp::Reply, warp::Rejection> {
	debug!("delete_todo: id={}", id);

	// if deleted {
	// 	Ok(StatusCode::NO_CONTENT)
	// } else {
	// 	Err(warp::reject::not_found())
	// } etc

	Ok(StatusCode::NO_CONTENT)
}
