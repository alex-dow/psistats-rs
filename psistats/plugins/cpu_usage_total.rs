use sysinfo::{System, SystemExt, RefreshKind};

pub fn cpu_usage_total() {

    let mut sys = System::new_with_specifics(RefreshKind::new());

    let procs = sys.get_processor_list();

    for proc in procs.iter().skip(0) {
        println!("proc: {:?}", proc);
    }
}
