use opencv::core::Point2f;
use anyhow::Result;

use crate::{detection::detector::Detector, image_fetcher::ImageFetcherClient};

const CORNER1_ID: i32 = 14;
const CORNER2_ID: i32 = 13;
const CORNER3_ID: i32 = 0;
const CORNER4_ID: i32 = 12;

const OUR_ID: i32 = 3;

pub struct ImgState {
    pub detector: Detector,
    pub corner1: Option<Point2f>,
    pub corner2: Option<Point2f>,
    pub corner3: Option<Point2f>,
    pub corner4: Option<Point2f>,
    pub our_position: Option<(Point2f, Point2f, Point2f, Point2f)>,
    pub enemies: Vec<(Point2f, Point2f, Point2f, Point2f)>,
    pub img_fetcher: ImageFetcherClient
}

impl ImgState {
    pub fn new(img_fetcher: ImageFetcherClient) -> Self {
        Self {
            detector: Detector::new(),
            img_fetcher: img_fetcher,
            corner1: None,
            corner2: None,
            corner3: None,
            corner4: None,
            our_position: None,
            enemies: Vec::new(),
        }
    }

    pub fn update(&mut self) -> Result<()> {
        let img = self.img_fetcher.get()?;
        let objects = self.detector.detect(&img);
        println!("{:?}", objects);

        Ok(())
    }
}

pub struct State {
    img1_state: ImgState,
    img2_state: ImgState,
}


impl Default for State {
    fn default() -> Self {
        Self {
            img1_state: ImgState::new(
                ImageFetcherClient::new(
                    "http://192.168.0.118:50051/frame", "983149"
                ),
            ),
            
            img2_state: ImgState::new(
                ImageFetcherClient::new(
                    "http://192.168.0.107:50051/frame", "378031"
                ),
            ),
        }
    }
}

pub enum StateEnum {
    Initializing,
    NotFound,
}

impl State {
    pub fn update(&mut self){
        self.img1_state.update();
        self.img2_state.update();
    }

    pub fn get_state(&self) -> StateEnum {
        StateEnum::Initializing   
    }
}


/// Angle (en degrés, entre -180 et 180) entre la direction de déplacement `start -> end`
/// et la direction `end -> target`.
/// Repère image (y vers le bas) : positif = la cible est à droite, négatif = à gauche.
/// Retourne `None` si `start` et `end` sont confondus (pas de direction).
pub fn heading_error(start: Point2f, end: Point2f, target: Point2f) -> Option<f32> {
    let (dx, dy) = (end.x - start.x, end.y - start.y);
    let (tx, ty) = (target.x - end.x, target.y - end.y);
    if dx == 0.0 && dy == 0.0 {
        return None;
    }
    let cross = dx * ty - dy * tx;
    let dot = dx * tx + dy * ty;
    Some(cross.atan2(dot).to_degrees())
}

/// Indique si le déplacement `start -> end` se dirige vers `target`,
/// avec une tolérance angulaire `tolerance_deg` (par ex. 20.0).
pub fn is_heading_towards(start: Point2f, end: Point2f, target: Point2f, tolerance_deg: f32) -> bool {
    heading_error(start, end, target).is_some_and(|angle| angle.abs() <= tolerance_deg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: f32, y: f32) -> Point2f {
        Point2f::new(x, y)
    }

    #[test]
    fn straight_ahead() {
        assert!(is_heading_towards(p(0.0, 0.0), p(10.0, 0.0), p(100.0, 5.0), 20.0));
    }

    #[test]
    fn behind() {
        assert!(!is_heading_towards(p(0.0, 0.0), p(10.0, 0.0), p(-100.0, 0.0), 20.0));
    }

    #[test]
    fn left_and_right() {
        // en allant vers la droite de l'image, une cible plus bas (y plus grand) est à droite
        assert!(heading_error(p(0.0, 0.0), p(10.0, 0.0), p(10.0, 100.0)).unwrap() > 0.0);
        assert!(heading_error(p(0.0, 0.0), p(10.0, 0.0), p(10.0, -100.0)).unwrap() < 0.0);
    }

    #[test]
    fn no_movement() {
        assert_eq!(heading_error(p(5.0, 5.0), p(5.0, 5.0), p(100.0, 0.0)), None);
        assert!(!is_heading_towards(p(5.0, 5.0), p(5.0, 5.0), p(100.0, 0.0), 20.0));
    }
}
