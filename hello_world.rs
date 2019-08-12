use rumqtt::{MqttClient, MqttOptions, QoS, Notification, ReconnectOptions};
use std::{thread, time::Duration};
use crossbeam_channel;
use sysinfo::{System, SystemExt};

mod psistats;

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
            /*
            println!("[heartbeat] resubscribing to all necessary topics");
            m1.subscribe("hello/world/commands", QoS::AtLeastOnce).unwrap();
            thread::sleep(Duration::from_secs(60));
            */
        }
    });

    let mqtt_rx2 = mqtt_rx.clone();
    let tx2 = tx.clone();
    let listening_thread = thread::spawn(move || {
        let short_sleep = Duration::from_millis(10);
        let long_sleep  = Duration::from_millis(1000);
        loop {
            psistats::brains::listen_tick(&short_sleep, &long_sleep, &mqtt_rx2, &tx2);
            /*
            if let Ok(notification) = mqtt_rx2.try_recv() {
                match notification {
                    Notification::Publish(publish) => {
                        let payload = String::from_utf8_lossy(&publish.payload);
                        let topic = publish.topic_name;
                        println!("message from \"{}\": {}", topic, payload);
                        let msg = format!("message from \"{}\": {}", topic, payload);
                        tx2.send(msg).unwrap();
                    },
                    _ => {}
                };
                thread::sleep(short_sleep);
            } else {
                thread::sleep(long_sleep);
            }
            */
        }
    });

    let mut m3 = mqtt_client.clone();
    let rx3 = rx.clone();
    let publisher_thread = thread::spawn(move || {
        let mut counter = 0;
        let sleep_time = Duration::from_millis(1000);
        let mut sys = System::new();
                
        loop {
            thread::sleep(sleep_time);
            sys.refresh_system();

            println!("processors: {:?}", sys.get_processor_list());


            // psistats::brains::publisher(&sleep_time, &mut m3, &rx3);

            /*           


            if let Ok(cmd) = rx3.try_recv() {
                println!("[publisher] rx3 reported: {:?}", cmd);
                counter += 1;

                let payload = format!("counter {}", counter);
                m3.publish("hello/world", QoS::AtLeastOnce, false, payload).unwrap();
            }

            

            let payload = format!("counter {}", counter);
            m1.publish("hello/world", QoS::AtMostOnce, false, payload).unwrap();
            counter += 1;
            */
        }
    });

    publisher_thread.join().unwrap();
    listening_thread.join().unwrap();
    heartbeat_thread.join().unwrap();

}

fn main() {
    pretty_env_logger::init();
    manager();
    /*
    let mqtt_options = MqttOptions::new("test-id", "127.0.0.1", 1883).set_keep_alive(30);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    let (done_tx, done_rx) = crossbeam_channel::bounded(1);

    mqtt_client.subscribe("hello/world", QoS::AtLeastOnce).unwrap();

    let sleep_time = Duration::from_millis(1000);

    thread::spawn(move || {
        for i in 0..1000 {
            let payload = format!("publish {}", i);
            thread::sleep(sleep_time);
            mqtt_client.subscribe("hello/world", QoS::AtLeastOnce).unwrap();
            mqtt_client.publish("hello/world", QoS::AtLeastOnce, false, payload).unwrap();
        }

        thread::sleep(sleep_time * 10);
        done_tx.send(true).unwrap();
    });

    loop {
        if let Ok(notification) = notifications.try_recv() {
            match notification {
                Notification::Publish(publish) => {
                    let payload = String::from_utf8_lossy(&publish.payload);
                    println!("publish notification: {:?}", payload);
                },
                _ => {}
            };
            
        }

    }
    */
}