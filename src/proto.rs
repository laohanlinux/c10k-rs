use serde_derive::{Serialize, Deserialize};
use serde_json::to_string;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proto {
    #[serde(rename = "content")]
    pub buf: serde_json::Value,
    #[serde(rename = "sender")]
    pub sender: u64,
    #[serde(rename = "receiver")]
    pub receiver: u64,
    #[serde(skip_serializing, skip_deserializing)]
    pub header: ProtoHeader,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ProtoHeader {
    pub operation: u32,
}

impl Proto {
    pub fn new(sender: u64) -> Self {
        Proto {
            buf: serde_json::Value::String("".to_string()),
            sender: sender,
            receiver: 0,
            header: ProtoHeader { operation: 0 },
        }
    }
}
