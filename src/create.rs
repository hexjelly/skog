use warp::http::StatusCode;

use super::{R2d2Pool, Status};

pub fn create_status(create: Status, _pool: R2d2Pool) -> Result<impl warp::Reply, warp::Rejection> {
	debug!("create_status: {:?}", create);

	Ok(StatusCode::CREATED)
}
