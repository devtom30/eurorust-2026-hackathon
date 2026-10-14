mod car;
mod goal;
<<<<<<< Updated upstream
mod detection;
=======

>>>>>>> Stashed changes
use goal::GoalClient;
use reqwest::blocking::Client;
use std::io::{self, Write};
use std::{thread, time::Duration};

fn get_current_location() -> String {
    // TODO: sem doplňte zjištění polohy (z kamery apod.)
    String::from("BL")
}

fn move_top(client: &Client) -> Result<(), reqwest::Error> {
    println!("Move top.");
    let max_attempts = 10;

    for attempt in 1..=max_attempts {
        match car::set_car(client, 0.5, false) {
            Ok(()) => return Ok(()),
            Err(e) if attempt < max_attempts => {
                eprintln!("Atempt {attempt}/{max_attempts} failed: {e}, trying again...");
                thread::sleep(Duration::from_millis(500));
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}

fn move_right() {
    thread::sleep(Duration::from_secs(1));
    println!("Move right.");
}

fn move_bottom() {
    thread::sleep(Duration::from_secs(1));
    println!("Move bottom.");
}

fn move_left() {
    thread::sleep(Duration::from_secs(1));
    println!("Move left.");
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
<<<<<<< Updated upstream
    let detector = detection::detector::Detector::new();
    let img = detector.read_image("uber/img1.png");
    let corners = detector.detect(&img);
    for corner in corners {
        println!("{:?}", corner);
    }
    
=======
    demo_movement();

>>>>>>> Stashed changes
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
            loc if loc == goal.quadrant => {
                println!("We are going to win! Right quadrant {}.", goal.quadrant)
            }
            _ => println!(
                "We have to move into right quadrant. ({current_location} -> {}, remainng {:.1} s)",
                goal.quadrant, goal.time_remaining
            ),
        }

        thread::sleep(Duration::from_secs(3));
    }
}
