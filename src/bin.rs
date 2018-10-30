extern crate pretty_env_logger;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate skog;
extern crate warp;
#[macro_use]
extern crate log;

use r2d2_sqlite::SqliteConnectionManager;
use skog::{create_status, delete_status, list_statuses};
use warp::Filter;

fn main() {
    pretty_env_logger::init();
    debug!("starting");
    let manager = SqliteConnectionManager::file("file.db");
    let pool = r2d2::Pool::new(manager).unwrap();
    let _conn = pool.get().unwrap(); // TODO: init db etc

    // Warp Filter for connection pool
    let pool = warp::any().map(move || pool.clone());

    // Warp Filter for accepting json body and limiting payload size
    let json_body = warp::body::content_length_limit(1024 * 128).and(warp::body::json());

    // base api point
    let base = warp::path("statuses");
    let statuses_index = base.and(warp::path::end());

    // specific status
    let statuses_id = base.and(warp::path::param::<u64>()).and(warp::path::end());

    // `GET /statuses`
    let list = warp::get2()
        .and(pool.clone())
        .and(statuses_index)
        .map(list_statuses);

    // `POST /statuses`
    let create = warp::post2()
        .and(statuses_index)
        .and(json_body)
        .and(pool.clone())
        .and_then(create_status);

    // `DELETE /statuses/:id`
    let delete = warp::delete2()
        .and(statuses_id)
        .and(pool.clone())
        .and_then(delete_status);

    // collect all api endpoints
    let api = list.or(create).or(delete);
    let log = warp::log("info");
    let routes = warp::path("api").and(warp::path("v1")).and(api).with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
    debug!("server running");
}
