#[cfg(test)]
mod magellan_test {
    use super::super::{ConfigData, MagellanInterface, ReportData};
    use tokio::io::AsyncReadExt;

    const TOKEN: &str = "29017ad4-51b6-47b4-a183-2d6dd38447e1";

    #[tokio::test]
    async fn report_test() {
        // Create a new MagellanInterface
        let magellan_interface = MagellanInterface::new(TOKEN.to_string());

        // Reporting a test message to the field
        let report_data = ReportData::Int("Test Report".to_string());
        magellan_interface
            .report("test-report", report_data)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn report_file_test() {
        // Create a new MagellanInterface
        let magellan_interface = MagellanInterface::new(TOKEN.to_string());

        // Read the file as byte
        let mut file_data = Vec::new();
        let mut file = tokio::fs::File::open("test_assets/test_file.txt")
            .await
            .unwrap();
        file.read_to_end(&mut file_data).await.unwrap();

        // Reporting an file
        let report_data = ReportData::TextFile(file_data);
        magellan_interface
            .report("test-file", report_data)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn config_test() {
        // Create a new MagellanInterface
        let magellan_interface = MagellanInterface::new(TOKEN.to_string());

        // Sending a configuration
        let config_data = ConfigData::Text("Test Config".to_string());
        magellan_interface
            .set_config("test-config", config_data)
            .await
            .unwrap();

        // Read the configuration
        let read_config = magellan_interface.read_config("test-config").await.unwrap();
        if let ConfigData::Text(data) = read_config {
            assert_eq!(data, "Test Config");
        } else {
            panic!("ConfigData is not a text variant");
        }
    }
}
