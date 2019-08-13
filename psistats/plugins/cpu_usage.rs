use sysinfo::{System, SystemExt, RefreshKind};
use std::thread;
use std::time::{Duration};
use crossbeam_channel::{Sender};


pub fn start_cpu_usage_thread(interval: u64, service_tx: &Sender<String>) {

    let tx = service_tx.clone();

    thread::spawn(move || {
        let st = Duration::from_millis(interval);
        let mut sys = System::new_with_specifics(RefreshKind::new());

        loop {
            sys.refresh_system();
            let procs = sys.get_processor_list();
            tx.send(format!("cpu: {:?}", procs)).unwrap();
            thread::sleep(st);
        }
    });
}