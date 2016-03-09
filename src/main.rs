#[macro_use] extern crate log;

mod logger;

fn main() {
    logger::init().unwrap();
    info!(target: "yak_events", "Commencing yak shaving for");
}
