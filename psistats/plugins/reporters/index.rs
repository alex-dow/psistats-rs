use super::cpu_usage::CpuUsage;
use super::mem_usage::MemUsage;
use super::ip_addrs::IpAddrs;
use super::reporter::Reporter;
use std::collections::HashMap;


pub fn get_all_reporters() -> HashMap<String, Box<Reporter + Send>> {
    let mut hm: HashMap<String, Box<Reporter + Send>> = HashMap::new();

    hm.insert("cpu-usage".to_string(), Box::new(CpuUsage::new()));
    hm.insert("mem-usage".to_string(), Box::new(MemUsage::new()));
    hm.insert("ip-addrs".to_string(), Box::new(IpAddrs::new()));

    return hm;
}