extern crate futures;

extern crate aodv;

use std::env::var;
use std::process::exit;

use aodv::{config, server};

fn main() {
    // Get command line arguments
    let args = config::get_args();

    // Start server
    if args.is_present("start_aodv") {
        // Check user is root
        match var("USER") {
            Ok(s) => {
                if s != "root" {
                    eprintln!("Must be root to run the server!");
                    exit(1);
                }
            }
            Err(e) => std::panic::panic_any(e),
        }

        // Start internal server
        server::aodv();
    }
}
