mod goal;
mod detection;
use goal::GoalClient;
use std::{thread, time::Duration};

fn get_current_location() -> String {
    // TODO: sem doplňte zjištění polohy (z kamery apod.)
    String::from("BL")
}

fn main() {
    let detector = detection::detector::Detector::new();
    let img = detector.read_image("uber/img1.png");
    let corners = detector.detect(&img);
    for corner in corners {
        println!("{:?}", corner);
    }
    
    let goals = GoalClient::new("http://192.168.0.19:8000/goal");

    loop {
        let goal = match goals.get() {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Error while reading the goal: {e}");
                thread::sleep(Duration::from_secs(1));
                continue;
            }
        };

        let current_location = get_current_location();

        match current_location.as_str() {
            "OUTSIDE_PICTURE" => println!("Something wrong."),
            "OUTSIDE_BOX" => println!("GET into box."),
            loc if loc == goal.quadrant => println!("We are going to win!"),
            _ => println!(
                "We have to move. ({current_location} -> {}, remainng {:.1} s)",
                goal.quadrant, goal.time_remaining
            ),
        }

        thread::sleep(Duration::from_secs(3));
    }
}
