use std::net::IpAddr;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "receipt-check")]
pub struct Options {
    #[structopt(short, long, default_value = "127.0.0.1")]
    address: IpAddr,

    #[structopt(short, long, default_value = "8080")]
    port: u16,

    #[structopt(short, long, parse(from_os_str), default_value = "db.sqlite")]
    database: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    key: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    certificate: PathBuf,
}

impl Options {
    pub fn address(&self) -> &IpAddr {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn database(&self) -> &Path {
        &self.database
    }

    pub fn key(&self) -> &Path {
        &self.key
    }

    pub fn certificate(&self) -> &Path {
        &self.certificate
    }
}
