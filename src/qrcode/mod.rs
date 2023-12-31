mod data;

pub use self::data::Reply;

use crate::database::Database;
use crate::ofd::load_params;
use crate::ofd::load_ticket;
use std::convert::Infallible;
use time::macros::format_description;

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

pub async fn qrcode(data: String, database: Database) -> Result<impl warp::Reply, Infallible> {
    info!("Request data: {}", data);

    let params = no_fail!("Failed to load ticket", load_params(&data).await);
    let key = params.key();
    let count = no_fail!(
        "Failed to count items",
        database.ticket_item_count(&key).await
    );

    if count > 0 {
        info!("Ticket already exists.");

        return Ok(warp::reply::json(&Reply::success()));
    }

    let ticket = no_fail!("Failed to load ticket", load_ticket(&params).await);
    let format = format_description!("[year].[month].[day]");
    let date_string = no_fail!("Failed to format date", ticket.datetime().format(&format));

    for item in ticket.items() {
        no_fail!(
            "Failed to save product",
            database
                .insert_ticket_item(&key, &date_string, item.name(), item.quantity(), item.sum())
                .await
        );
    }

    Ok(warp::reply::json(&Reply::success()))
}
