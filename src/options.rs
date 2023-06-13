use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "receipt-check")]
pub struct Options {
    #[structopt(short, long, default_value = "127.0.0.1")]
    address: String,

    #[structopt(short, long, default_value = "8080")]
    port: u16,

    #[structopt(short, long, parse(from_os_str), default_value = "db.sqlite")]
    database: PathBuf,
}

impl Options {
    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn database(&self) -> &Path {
        &self.database
    }
}
