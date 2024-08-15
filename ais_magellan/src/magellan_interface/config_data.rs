use serde_json::Value;

/// Configuration to be sent to the AIS Magellan server
pub enum ConfigData {
    Text(String),
    Int(i32),
    Double(f64),
    Boolean(bool),
}

impl ConfigData {
    /// Preprocess the data to be sent to the server
    pub fn serialize(self) -> Value {
        match self {
            ConfigData::Text(data) => Value::from(data),
            ConfigData::Int(data) => Value::from(data),
            ConfigData::Double(data) => Value::from(data),
            ConfigData::Boolean(data) => Value::from(data),
        }
    }

    // Construct a ConfigData from a Value
    pub fn from_value(value: Value) -> Self {
        match value {
            Value::String(data) => ConfigData::Text(data),
            Value::Number(data) => {
                if data.is_i64() {
                    ConfigData::Int(data.as_i64().unwrap() as i32)
                } else {
                    ConfigData::Double(data.as_f64().unwrap())
                }
            }
            Value::Bool(data) => ConfigData::Boolean(data),
            _ => panic!("Invalid ConfigData variant"),
        }
    }
}
