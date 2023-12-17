mod nt;

use std::env;

use nt::config::Path;

use crate::nt::{NTConfig, NT};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let host = args[1].clone();
    let port = args[2].parse::<usize>().unwrap();

    let mut cfg = NTConfig::new(host, port);

    cfg.with(Path {
        src: String::from("127.0.0.1"),
        dest: String::from("127.0.0.1"),
    });

    let mut nt = NT::new(cfg);
    nt.start().await.unwrap_or_else(|err| {
        println!("Failed with {:?}", err);
        return false;
    });
}
