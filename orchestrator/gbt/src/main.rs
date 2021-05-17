#[macro_use]
extern crate log;

use args::Opts;
use clap::{App, Clap};
use env_logger::Env;

mod args;
//mod eth_to_cosmos;
mod utils;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // On Linux static builds we need to probe ssl certs path to be able to
    // do TLS stuff.
    openssl_probe::init_ssl_cert_env_vars();
    // parse the arguments
    let opts: Opts = Opts::parse();

    // this may be unreachable
    panic!("No valid subcommand found!");
}
