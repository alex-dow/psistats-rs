use serde_json::json;
use super::reporter;
use get_if_addrs::get_if_addrs;

pub struct IpAddrs {
    from: String
}

impl IpAddrs {
    pub fn new() -> IpAddrs {
        IpAddrs { 
            from: "ip-addrs".to_string()
        }
    }
}

impl reporter::Reporter for IpAddrs {
    fn make_report(&mut self) -> reporter::Report {

        let mut ips = vec![];

        for iface in get_if_addrs().unwrap() {
            let ip = (iface.name, iface.addr.ip());
            ips.push(ip);
        }

        return reporter::Report {
            reporter: self.from.clone(),
            message: json!(ips),
            hostname: "".to_string()
        };
    }
}
