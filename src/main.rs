#[macro_use]
extern crate log;

#[macro_use]
extern crate time;

mod ofd;

use ofd::load_ticket;
use serde::Deserialize;
use serde::Serialize;
use std::convert::Infallible;
use time::macros::format_description;
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Create routes...");

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("public/index.html"));
    let public = warp::get().and(warp::fs::dir("public"));
    let qrcode = warp::path!("api" / "qrcode")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(|| "test"))
        .and_then(get_ticket);
    let routes = index.or(qrcode).or(public);

    info!("Starting server...");

    warp::serve(routes)
        .tls()
        .cert_path("tls/certificate.pem")
        .key_path("tls/key.pem")
        .run(([0, 0, 0, 0], 8081))
        .await;
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

async fn get_ticket(data: String, _test: &str) -> Result<impl warp::Reply, Infallible> {
    info!("Request data: {}", data);

    let response = match load_ticket(&data).await {
        Ok(ticket) => {
            let format = format_description!("[year].[month].[day]");
            let date_string = ticket.datetime().format(&format).unwrap();

            for item in ticket.items() {
                println!(
                    "{};{};{:0.3};{:0.2}",
                    date_string,
                    item.name(),
                    item.quantity(),
                    item.sum()
                );
            }

            TicketReply::success()
        }
        Err(error) => {
            warn!("Failed to parse ticket: {}", error);

            let message = format!("{}", error);

            TicketReply::error(&message)
        }
    };

    Ok(warp::reply::json(&response))
}
