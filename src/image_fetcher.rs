use reqwest::blocking::Client;
use anyhow::Result;
use crate::detection::detector::Img;


pub struct ImageFetcherClient {
    client: Client,
    url: String, 
    auth: String,
}

impl ImageFetcherClient {
    pub fn new(url: &str, auth: &str) -> Self {
        Self { client: Client::new(), url: url.to_string(), auth: auth.to_string() }
    }

    pub fn get(&self) -> Result<Img> {
        let response = self.client
            .get(&self.url)
            .header("Authorization", &self.auth)
            .send()?
            .error_for_status()?;
        let bytes = response
            .bytes()?;
        
        Ok(Img::try_from(bytes.as_ref())?)
    }
}
