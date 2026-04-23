use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_credential_types::Credentials;
use aws_sdk_s3::Client;

pub struct GarageClient {
    client: Option<Client>,
}

impl GarageClient {
    pub fn new() -> Self {
        GarageClient {
            client: None,
        }
    }

    pub async fn init(
        &mut self,
        access_key: String,
        secret_key: String,
        region: String,
        host: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let credentials = Credentials::new(access_key, secret_key, None, None, "garage-static");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(region))
            .credentials_provider(credentials)
            .load()
            .await;
        let s3_config = aws_sdk_s3::config::Builder::from(&config)
            // .endpoint_url("http://127.0.0.1:3900")
            .endpoint_url(host)
            .force_path_style(true)
            .build();
        self.client = Some(Client::from_conf(s3_config));
        Ok(())
    }

    pub fn get_client(&self) -> Result<&Client, Box<dyn std::error::Error>> {
        match self.client.as_ref() {
            None => {Err("Database not initialized".into())}
            Some(client) => Ok(client)
        }
    }
}
