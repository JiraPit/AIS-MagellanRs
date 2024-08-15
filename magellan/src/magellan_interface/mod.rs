pub mod config_data;
pub mod report_data;
mod test;

use config_data::ConfigData;
use report_data::ReportData;
use reqwest::Client;
use serde_json::Value;
use std::{collections::HashMap, error::Error};

const ROOT: &str = "https://magellan.ais.co.th/asgardhttpv2/api/v2";

/// Interface to the AIS Magellan server
pub struct MagellanInterface {
    token: String,
    client: Client,
}

impl MagellanInterface {
    pub fn new(token: String) -> Self {
        let client = Client::new();
        Self { token, client }
    }

    /// Report a key-value pair to the AIS Magellan
    pub async fn report(&self, key: &str, value: ReportData) -> Result<(), Box<dyn Error>> {
        //Construct the map to send to the server
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert(key.to_string(), value.serialize());

        // Send the data to the server
        let response = self
            .client
            .post(format!("{ROOT}/thing/report"))
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&map)
            .send()
            .await?;

        // Check if the request was successful
        if response.status().is_success() {
            Ok(())
        } else {
            // If not, read and return the error message
            let status_code = response.status();
            let value = response.json::<Value>().await?;
            let ais_error_code = match value.get("Code") {
                Some(code) => code.to_string(),
                None => "No error code".to_string(),
            };
            Err(format!(
                "Error reporting to AIS Magellan: {}, {}",
                status_code, ais_error_code
            )
            .into())
        }
    }

    /// Set a config on AIS Magellan
    pub async fn set_config(&self, key: &str, value: ConfigData) -> Result<(), Box<dyn Error>> {
        // Construct the map to send to the servers
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert(key.to_string(), value.serialize());

        // Send the request to the server
        let response = self
            .client
            .post(format!("{ROOT}/thing/config"))
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&map)
            .send()
            .await?;

        // Check if the request was successful
        if response.status().is_success() {
            Ok(())
        } else {
            // If not, read and return the error message
            let status_code = response.status();
            let value = response.json::<Value>().await?;
            let ais_error_code = match value.get("Code") {
                Some(code) => code.to_string(),
                None => "No error code".to_string(),
            };
            Err(format!(
                "Error reporting to AIS Magellan: {}, {}",
                status_code, ais_error_code
            )
            .into())
        }
    }

    /// Read a config from AIS Magellan
    pub async fn read_config(&self, key: &str) -> Result<ConfigData, Box<dyn Error>> {
        // Send the request to the server
        let response = self
            .client
            .post(format!("{ROOT}/thing/config"))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        // Check if the request was successful
        if response.status().is_success() {
            let value = response.json::<Value>().await?;
            let value = value["ClientConfig"][key].clone();
            let config = ConfigData::from_value(value);
            Ok(config)
        } else {
            // If not, read and return the error message
            let status_code = response.status();
            let value = response.json::<Value>().await?;
            let ais_error_code = match value.get("Code") {
                Some(code) => code.to_string(),
                None => "No error code".to_string(),
            };
            Err(format!(
                "Error reporting to AIS Magellan: {}, {}",
                status_code, ais_error_code
            )
            .into())
        }
    }

    /// Read all configs from AIS Magellan
    pub async fn read_all_configs(&self) -> Result<HashMap<String, ConfigData>, Box<dyn Error>> {
        // Send the request to the server
        let response = self
            .client
            .post(format!("{ROOT}/thing/config"))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        // Check if the request was successful
        if response.status().is_success() {
            let value = response.json::<Value>().await?;
            let value = value["ClientConfig"].clone();
            let mut map: HashMap<String, ConfigData> = HashMap::new();
            for (k, v) in value.as_object().unwrap() {
                map.insert(k.to_string(), ConfigData::from_value(v.clone()));
            }
            Ok(map)
        } else {
            // If not, read and return the error message
            let status_code = response.status();
            let value = response.json::<Value>().await?;
            let ais_error_code = match value.get("Code") {
                Some(code) => code.to_string(),
                None => "No error code".to_string(),
            };
            Err(format!(
                "Error reporting to AIS Magellan: {}, {}",
                status_code, ais_error_code
            )
            .into())
        }
    }
}
