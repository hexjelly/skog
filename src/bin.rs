extern crate pretty_env_logger;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate skog;
#[macro_use]
extern crate warp;
#[macro_use]
extern crate log;

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::NO_PARAMS;
use skog::{create_status, delete_status, get_status, list_statuses, update_status};
use warp::Filter;

fn main() {
    pretty_env_logger::init();
    debug!("starting");
    let manager = SqliteConnectionManager::file("file.db");
    let pool = r2d2::Pool::new(manager).expect("error creating connection pool");
    let conn = pool.get().expect("error getting connection from pool");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS statuses (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            date        INTEGER,
            update_date INTEGER,
            eta         INTEGER,
            close_date  INTEGER,
            title       TEXT,
            message     TEXT
        )",
        NO_PARAMS,
    ).expect("unable to initialize db table");

    // Warp Filter for connection pool
    let pool = warp::any().map(move || pool.clone());

    // Warp Filter for accepting json body and limiting payload size
    let json_body = warp::body::content_length_limit(1024 * 128).and(warp::body::json());

    // base api point
    let base = warp::path("statuses");
    let statuses_index = base.and(warp::path::end());

    // specific status
    let statuses_id = base.and(warp::path::param::<i64>()).and(warp::path::end());

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

    // `GET /statuses/:id`
    let get = warp::get2()
        .and(statuses_id)
        .and(pool.clone())
        .and_then(get_status);

    // `DELETE /statuses/:id`
    let delete = warp::delete2()
        .and(statuses_id)
        .and(pool.clone())
        .and_then(delete_status);

    // `PUT /statuses/:id`
    let update = warp::put2()
        .and(statuses_id)
        .and(json_body)
        .and(pool.clone())
        .and_then(update_status);

    // collect all api endpoints
    let log = warp::log("info");
    let api_base = path!("api" / "v1");
    let api = api_base.and(list.or(create).or(get).or(delete).or(update));

    let routes = api.with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
    debug!("server running");
}
