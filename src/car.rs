use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize)]
struct CarCommand {
    speed: f64,
    flip: bool,
}

pub fn set_car(client: &Client, speed: f64, flip: bool) -> Result<(), reqwest::Error> {
    client
        .put("http://hackathon-3-car.local:5000")
        .header("Authorization", "838748")
        .json(&CarCommand { speed, flip })
        .send()?
        .error_for_status()?;
    Ok(())
}