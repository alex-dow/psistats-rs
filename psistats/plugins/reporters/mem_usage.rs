use sysinfo::{System, SystemExt, RefreshKind};
use serde_json::json;
use super::reporter;

pub struct MemUsage {
    sys: System,
    from: String
}

impl MemUsage {
    pub fn new() -> MemUsage {
        MemUsage { 
            sys: System::new_with_specifics(RefreshKind::new()),
            from: "mem-usage".to_string()
        }
    }
}

impl reporter::Reporter for MemUsage {
    fn make_report(&mut self) -> reporter::Report {
        self.sys.refresh_system();

        let msg = vec![self.sys.get_total_memory(), self.sys.get_free_memory()];
        let output = json!(msg);

        return reporter::Report {
            reporter: self.from.clone(),
            hostname: "".to_string(),
            message: output
        };
    }
}