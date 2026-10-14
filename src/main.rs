mod goal;
mod detection;
mod image_fetcher;
use goal::GoalClient;
use image_fetcher::ImageFetcherClient;
use std::{thread, time::Duration};

fn get_current_location() -> String {
    // TODO: sem doplňte zjištění polohy (z kamery apod.)
    String::from("BL")
}

fn main() {
    let detector = detection::detector::Detector::new();

    let camera1 = ImageFetcherClient::new("http://192.168.0.118:50051/frame", "983149");
    let camera2 = ImageFetcherClient::new("http://192.168.0.107:50051/frame", "378031");
    
    let goals = GoalClient::new("http://192.168.0.19:8000/goal");

    loop {
        let img1 = camera1.get().unwrap();
        let corners1 = detector.detect(&img1);
        let img2 = camera2.get().unwrap();
        let corners2 = detector.detect(&img2);

        for corner in corners1.iter().zip(corners2) {
            println!("{:?}", corner);
        }
        
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
