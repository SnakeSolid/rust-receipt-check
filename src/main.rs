#[macro_use]
extern crate log;

mod database;
mod ofd;

use database::Database;
use ofd::load_params;
use ofd::load_ticket;
use serde::Deserialize;
use serde::Serialize;
use std::convert::Infallible;
use std::error::Error;
use time::macros::format_description;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("Create routes...");

    let database = Database::new("db.sqlite")?;

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("public/index.html"));
    let public = warp::get().and(warp::fs::dir("public"));
    let qrcode = warp::path!("api" / "qrcode")
        .and(warp::post())
        .and(warp::body::json())
        .and(with(database.clone()))
        .and_then(get_ticket);
    let routes = index.or(qrcode).or(public);

    info!("Starting server...");

    warp::serve(routes)
        .tls()
        .cert_path("tls/certificate.pem")
        .key_path("tls/key.pem")
        .run(([0, 0, 0, 0], 8081))
        .await;

    Ok(())
}

fn with(value: Database) -> impl Filter<Extract = (Database,), Error = Infallible> + Clone {
    warp::any().map(move || value.clone())
}

#[derive(Debug, Serialize, Deserialize)]
enum TicketReply {
    Success,
    Error { message: String },
}

impl TicketReply {
    pub fn success() -> Self {
        Self::Success
    }

    pub fn error(message: &str) -> Self {
        Self::Error {
            message: message.into(),
        }
    }
}

macro_rules! no_fail {
    ($message:expr, $callback:expr) => {
        match $callback {
            Ok(result) => result,
            Err(error) => {
                warn!("{}: {}", $message, error);

                let message = format!("{}", error);

                return Ok(warp::reply::json(&TicketReply::error(&message)));
            }
        }
    };
}

async fn get_ticket(data: String, database: Database) -> Result<impl warp::Reply, Infallible> {
    info!("Request data: {}", data);

    let params = no_fail!("Failed to load ticket", load_params(&data).await);
    let key = params.key();
    let count = no_fail!(
        "Failed to count items",
        database.ticket_item_count(&key).await
    );

    if count > 0 {
        info!("Ticket already exists.");

        return Ok(warp::reply::json(&TicketReply::success()));
    }

    let ticket = no_fail!("Failed to load ticket", load_ticket(&params).await);
    let format = format_description!("[year].[month].[day]");
    let date_string = no_fail!("Failed to format date", ticket.datetime().format(&format));

    for item in ticket.items() {
        no_fail!(
            "Failed to save product",
            database
                .insert_ticket_item(&key, item.name(), item.quantity(), item.sum())
                .await
        );

        let category_name = no_fail!(
            "Failed to query product",
            database.category_name(item.name()).await
        );

        if let Some(category_name) = category_name {
            println!(
                "{};{};{};{:0.3};{:0.2}",
                date_string,
                category_name.category(),
                category_name.name(),
                item.quantity(),
                item.sum(),
            );
        } else {
            println!(
                "{};{};;{:0.3};{:0.2}",
                date_string,
                item.name(),
                item.quantity(),
                item.sum()
            );
        }
    }

    Ok(warp::reply::json(&TicketReply::success()))
}
