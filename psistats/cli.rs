mod workers;
use std::thread;

fn manager() {
    let reporters = thread::spawn(|| {
        workers::start();
    });

    let _ = reporters.join();
}


/*
fn manager() {
    
    let mqtt_options = MqttOptions::new("test-id", "127.0.0.1", 1883)
        .set_keep_alive(120)
        .set_reconnect_opts(ReconnectOptions::Always(5));
        
    let (mqtt_client, mqtt_rx) = MqttClient::start(mqtt_options).unwrap();

    let (tx, rx) = crossbeam_channel::bounded::<String>(5);



    let mut m1 = mqtt_client.clone();
    let heartbeat_thread = thread::spawn(move || {
        loop {
            psistats::brains::heartbeat_tick(10, &mut m1);
        }
    });

    let mqtt_rx2 = mqtt_rx.clone();
    let tx2 = tx.clone();
    let listening_thread = thread::spawn(move || {
        let short_sleep = Duration::from_millis(10);
        let long_sleep  = Duration::from_millis(1000);
        loop {
            psistats::brains::listen_tick(&short_sleep, &long_sleep, &mqtt_rx2, &tx2);
        }
    });

    let tx3 = tx.clone();
    let rx3 = rx.clone();
    let publisher_thread = thread::spawn(move || {
        psistats::brains::publisher(&rx3, &tx3);
    });

    publisher_thread.join().unwrap();
    listening_thread.join().unwrap();
    heartbeat_thread.join().unwrap();

}

*/

fn main() {
    pretty_env_logger::init();
    manager();
}