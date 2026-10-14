use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Goal {
    pub quadrant: String,
    pub time_remaining: f64,
}

pub struct GoalClient {
    client: Client,
    url: String,
}

impl GoalClient {
    pub fn new(url: &str) -> Self {
        Self { client: Client::new(), url: url.to_string() }
    }

    pub fn get(&self) -> Result<Goal, reqwest::Error> {
        self.client
            .get(&self.url)
            .send()?
            .error_for_status()?
            .json()
    }
}