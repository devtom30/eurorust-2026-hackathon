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

