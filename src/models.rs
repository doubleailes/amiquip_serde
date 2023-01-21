#![crate_type = "lib"]
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct FibPayload {
    pub n: u64,
}
pub fn deser_message(body: String) -> FibPayload {
    serde_json::from_str(&body[..]).unwrap()
}
#[derive(Deserialize, Serialize)]
pub struct FibResult {
    pub result: u64,
}
pub fn ser_message(value: u64) -> String {
    serde_json::to_string(&FibResult { result: value }).unwrap()
}
