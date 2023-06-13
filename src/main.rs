#[macro_use]
extern crate log;

mod categories;
mod database;
mod ofd;
mod options;
mod qrcode;
mod tickets;

use crate::database::Database;
use options::Options;
use std::convert::Infallible;
use std::error::Error;
use std::net::IpAddr;
use std::str::FromStr;
use structopt::StructOpt;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("Parsing options...");

    let options = Options::from_args();

    info!("Setup database...");

    let database = Database::new(options.database())?;

    info!("Create routes...");

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("public/scanner.html"));
    let qrcode = warp::path!("api" / "qrcode")
        .and(warp::post())
        .and(warp::body::json())
        .and(with(database.clone()))
        .and_then(qrcode::qrcode);
    let tickets_list = warp::path!("api" / "tickets" / "list")
        .and(warp::post())
        .and(with(database.clone()))
        .and_then(tickets::list);
    let tickets_clear = warp::path!("api" / "tickets" / "clear")
        .and(warp::post())
        .and(with(database.clone()))
        .and_then(tickets::clear);
    let categories_list = warp::path!("api" / "categories" / "list")
        .and(warp::post())
        .and(with(database.clone()))
        .and_then(categories::list);
    let categories_update = warp::path!("api" / "categories" / "update")
        .and(warp::post())
        .and(warp::body::json())
        .and(with(database.clone()))
        .and_then(categories::update);
    let public = warp::get().and(warp::fs::dir("public"));
    let routes = index
        .or(qrcode)
        .or(tickets_list)
        .or(tickets_clear)
        .or(categories_list)
        .or(categories_update)
        .or(public);

    info!("Starting server...");

    warp::serve(routes)
        .tls()
        .cert_path("tls/certificate.pem")
        .key_path("tls/key.pem")
        .run((IpAddr::from_str(options.address())?, options.port()))
        .await;

    Ok(())
}

fn with(value: Database) -> impl Filter<Extract = (Database,), Error = Infallible> + Clone {
    warp::any().map(move || value.clone())
}
