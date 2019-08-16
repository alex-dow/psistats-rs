use std::thread;
use std::time::Duration;
use std::env;
use std::fs;

#[path = "plugins/reporters/mod.rs"]
mod reporters;

use toml::Value;
use toml;
use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use serde_json;
use hostname;

pub fn start() {

    let (reporter_tx, reporter_rx) = unbounded();
    let (reporter_tx1, reporter_rx1) = (reporter_tx.clone(), reporter_rx.clone());

    let (commander_tx, commander_rx) = unbounded();
    let (commander_tx1, commander_rx1) = (commander_tx.clone(), commander_rx.clone());

    let name = hostname::get_hostname().unwrap();

    let reporters_thread = thread::spawn(move || {
        start_reporters(reporter_tx1, commander_rx1);
    });

    thread::spawn(move || {
        let x = Duration::from_secs(10);
        thread::sleep(x);
        commander_tx1.send("ip-addrs".to_string()).unwrap();
    });

    loop {
        let mut msg = reporter_rx.recv().unwrap();
        msg.hostname = name.clone();
        let json = serde_json::to_string(&msg).unwrap();
        println!("{}", json);
    }

    let _ = reporters_thread.join();
}

pub fn start_reporters(reporter_tx: Sender<reporters::Report>, commander_rx: Receiver<String>) {
    let r_plugins = reporters::get_all_reporters();

    let mut configfile = env::current_dir().unwrap();
    configfile.push("psistats.toml");

    let contents = fs::read_to_string(configfile).unwrap();
    let config   = contents.parse::<Value>().unwrap();

    let mut reporter_threads = vec![];

    for (reporter_name, mut reporter) in r_plugins {
        let r_config = &config[&reporter_name];

        let sleep_time = config[&reporter_name]["interval"].as_integer().unwrap() as u64;

        let reporter_tx_clone = reporter_tx.clone();
        let commander_rx_clone = commander_rx.clone();
        reporter_threads.push(thread::spawn(move || {
            if (sleep_time == 0) {
                loop {
                    let cmd = commander_rx_clone.recv().unwrap();
                    if (cmd == reporter_name) {
                        let r = reporter.make_report();
                        reporter_tx_clone.send(r).unwrap();
                    }
                    println!("reporter thread receieved command: {:?}", cmd);
                }
            } else {
                let d = Duration::from_secs(sleep_time);

                loop {
                    let r = reporter.make_report();
                    reporter_tx_clone.send(r).unwrap();
                    thread::sleep(d);
                }
            }
        }));
    }

    for t in reporter_threads {
        let _ = t.join();
    }

}
