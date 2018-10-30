#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate serde;
extern crate warp;
#[macro_use]
extern crate log;

use r2d2_sqlite::SqliteConnectionManager;

pub type R2d2Pool = r2d2::Pool<SqliteConnectionManager>;

mod create;
mod delete;
mod list;
mod status;
pub use create::create_status;
pub use delete::delete_status;
pub use list::list_statuses;
pub use status::Status;
