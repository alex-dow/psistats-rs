use serde_json::value::Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub reporter: String,
    pub hostname: String,
    pub message: Value
}

pub trait Reporter {
    fn make_report(&mut self) -> Report;
}

