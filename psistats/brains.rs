use std::thread;
use std::time::Duration;
use rumqtt::{MqttClient, QoS, Notification};
use crossbeam_channel::{Receiver, Sender};
use psistats::plugins::cpu_usage::start_cpu_usage_thread;
use psistats::plugins::mem_usage::start_mem_usage_thread;

use crate::psistats;

pub fn heartbeat_tick(sleep_time: u64, mqtt_client:&mut MqttClient) {
    println!("[heartbeat] tick");
    mqtt_client.subscribe("hello/world/commands", QoS::AtLeastOnce).unwrap();
    thread::sleep(Duration::from_secs(sleep_time));
}

pub fn listen_tick(short_sleep_time: &Duration, long_sleep_time: &Duration, mqtt_rx: &Receiver<Notification>, service_tx: &Sender<String>) {

    if let Ok(notification) = mqtt_rx.try_recv() {
        match notification {
            Notification::Publish(publish) => {
                let payload = String::from_utf8_lossy(&publish.payload);
                let topic = publish.topic_name;
                let msg = format!("[listen] message from \"{}\": {}", topic, payload);
                println!("{}", msg);
                service_tx.send(msg).unwrap();
            },
            _ => {}
        };
        thread::sleep(*short_sleep_time);
    } else {
        thread::sleep(*long_sleep_time);
    }
}

pub fn publisher(service_rx: &Receiver<String>, service_tx: &Sender<String>) {
    //if let Ok(cmd) = service_rx.try_recv() {
        start_cpu_usage_thread(1000, service_tx);
        start_mem_usage_thread(5000, service_tx);

        loop {
            let msg = service_rx.recv();
            match msg {
                Ok(m) => println!("[publisher] {:?}", m),
                Err(_) => {}
            };
        }
        //psistats::plugins::cpu_usage_total();
        //thread::sleep(*sleep_time);
        //println!("[publisher] service_rx reported: {:?}", cmd);
    //}
}