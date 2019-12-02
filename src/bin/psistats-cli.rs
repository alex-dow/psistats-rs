#![allow(dead_code)]
#![allow(unused_variables)]

use crossbeam_channel as channel;
// use psistats::config::Config;
use psistats::manager::{start_reporters, start_publishers};
use psistats::reporters::Report;
use std::fs;
use std::env;
use pretty_env_logger;
use toml;
#[macro_use] extern crate log;

fn main() {

    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    info!("Starting psistats");

    let conf_contents = fs::read_to_string("psistats.toml")
        .expect("Error reading psistats.toml");

    let config = conf_contents.parse::<toml::Value>().unwrap();
    

    let (reports_sender, reports_receiver) = channel::unbounded::<Report>();
    let (commands_sender, commands_receiver) = channel::unbounded::<String>();

    let reporter_proc = start_reporters(config.clone(), reports_sender.clone(), commands_receiver.clone());
    let publisher_proc = start_publishers(config.clone(), reports_receiver.clone(), commands_sender.clone());

    let _ = reporter_proc.join();
    // let _ = publisher_proc.join();
}
