mod car;
mod goal;
mod detection;
mod image_fetcher;

use goal::GoalClient;
use detection::navigation::{State, StateEnum};
use reqwest::blocking::Client;
use std::io::{self, Write};
use std::{thread, time::Duration};

fn get_current_location() -> String {
    // TODO: sem doplňte zjištění polohy (z kamery apod.)
    String::from("BL")
}

fn drive(speed: f64, flip: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let body = format!(r#"{{"speed":{speed},"flip":{flip}}}"#);

    let resp = client
        .put("http://192.168.0.149:5000")
        .header("Content-Type", "application/json")
        .header("Authorization", "838748")
        .body(body)
        .send()?;

    println!("Status: {}", resp.status());
    println!("{}", resp.text()?);
    Ok(())
}

fn drive_direction(direction: &str) -> Result<(), Box<dyn std::error::Error>> {
    match direction {
        "forward" => drive(1.0, false)?,
        "backward" => drive(-1.0, false)?,
        "turn_left" => drive(0.2, true)?,
        "turn_right" => drive(-0.2, true)?,
        _ => println!("Unknown direction: {direction}"),
    }
    Ok(())
}

fn move_into_defaut_state() {
    // TODO do it yourself
    println!("Do it yourself...");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
}

fn demo_movement() -> Result<(), Box<dyn std::error::Error>> {
    move_into_defaut_state();
    let client = Client::builder().timeout(Duration::from_secs(3)).build()?;

    if let Err(e) = move_top(&client) {
        eprintln!("Move top failed: {e}");
    }
    move_right();
    move_bottom();
    move_left();
    Ok(())
}

fn main() {
    let mut state = State::default();
    //demo_movement();

    let goals = GoalClient::new("http://192.168.0.19:8000/goal");

    loop {
        state.update();
        let goal = match goals.get() {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Error while reading the goal: {e}");
                thread::sleep(Duration::from_secs(1));
                continue;
            }
        };

        let current_location = get_current_location();

        match state.get_state() {
            StateEnum::Initializing => println!("Init positions, Figuring out"),
            StateEnum::NotFound => println!("Car is probably outside the camera!!!"),
            //loc if loc == goal.quadrant => {
            //    println!("We are going to win! Right quadrant {}.", goal.quadrant)
            //}
            _ => println!(
                "We have to move into right quadrant. ({current_location} -> {}, remainng {:.1} s)",
                goal.quadrant, goal.time_remaining
            ),
        }

        thread::sleep(Duration::from_secs(3));
    }
}
