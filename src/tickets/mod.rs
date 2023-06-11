mod data;

pub use self::data::Reply;
pub use self::data::ReplyItem;

use crate::database::Database;
use std::convert::Infallible;

macro_rules! no_fail {
    ($message:expr, $callback:expr) => {
        match $callback {
            Ok(result) => result,
            Err(error) => {
                warn!("{}: {}", $message, error);

                let message = format!("{}", error);

                return Ok(warp::reply::json(&Reply::error(&message)));
            }
        }
    };
}

pub async fn tickets(database: Database) -> Result<impl warp::Reply, Infallible> {
    info!("Request tickets");

    let items = no_fail!("Failed to read items", database.select_ticket_items().await);

    Ok(warp::reply::json(&Reply::success(items)))
}
