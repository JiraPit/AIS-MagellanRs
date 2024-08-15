use base64::{prelude::BASE64_URL_SAFE as base64_encoder, Engine};
use serde_json::Value;

/// Data to be reported to the AIS Magellan server
pub enum ReportData {
    Int(String),
    Number(i32),
    Double(f64),
    Boolean(bool),
    Image(Vec<u8>),
    Video(Vec<u8>),
    TextFile(Vec<u8>),
    Audio(Vec<u8>),
}

impl ReportData {
    /// Serialize the data to be sent to the server
    pub fn serialize(self) -> Value {
        match self {
            ReportData::Int(data) => Value::from(data),
            ReportData::Number(data) => Value::from(data),
            ReportData::Double(data) => Value::from(data),
            ReportData::Boolean(data) => Value::from(data),
            ReportData::Image(data) => {
                let encoded = base64_encoder.encode(data);
                let encoded = format!("data:image/png;base64,{}", encoded);
                Value::from(encoded)
            }
            ReportData::Video(data) => {
                let encoded = base64_encoder.encode(data);
                let encoded = format!("data:video/mp4;base64,{}", encoded);
                Value::from(encoded)
            }
            ReportData::TextFile(data) => {
                let encoded = base64_encoder.encode(data);
                let encoded = format!("data:text/plain;base64,{}", encoded);
                Value::from(encoded)
            }
            ReportData::Audio(data) => {
                let encoded = base64_encoder.encode(data);
                let encoded = format!("data:audio/mpeg;base64,{}", encoded);
                Value::from(encoded)
            }
        }
    }
}
