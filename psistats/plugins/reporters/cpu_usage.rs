use sysinfo::{System, SystemExt, RefreshKind, ProcessorExt};
use serde_json::json;
use super::reporter;

pub struct CpuUsage {
    sys: System,
    from: String
}

impl CpuUsage {
    pub fn new() -> CpuUsage {
        CpuUsage { 
            sys: System::new_with_specifics(RefreshKind::new()),
            from: "cpu".to_string()
        }
    }
}

impl reporter::Reporter for CpuUsage {
    fn make_report(&mut self) -> reporter::Report {
        self.sys.refresh_system();
        let procs = self.sys.get_processor_list();

        let msg: Vec<f32> = procs.iter().map(|p| {
            return p.get_cpu_usage();
        }).collect();

        let output = json!(msg);

        return reporter::Report {
            reporter: self.from.clone(),
            message: output,
            hostname: "".to_string()
        }
    }
}
