use opencv::{
    aruco,
    core::{Mat, Point2f, Ptr, Vector},
    imgcodecs,
};
use std::{collections::HashMap};

pub struct Detector {
    dictionary: Ptr<aruco::Dictionary>,
    parameters: Ptr<aruco::DetectorParameters>,
}

pub struct Img {
    mat: Mat,
}

impl TryFrom<&[u8]> for Img {
    type Error = opencv::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let buf = Vector::<u8>::from_slice(value);
        let mat = imgcodecs::imdecode(&buf, imgcodecs::IMREAD_COLOR)?;
        Ok(Img { mat })

    }
}

impl Detector {

    pub fn new() -> Self {
        let dictionary =
            aruco::get_predefined_dictionary(aruco::PREDEFINED_DICTIONARY_NAME::DICT_4X4_50).unwrap();
        let parameters = aruco::DetectorParameters::create().unwrap();

        Self { dictionary, parameters }
    }

    pub fn detect(&self, img: &Img) -> HashMap<i32, Vector<Point2f>> {
        let mut corners = Vector::<Vector<Point2f>>::new();
        let mut ids = Vector::<i32>::new();
        let mut rejected = Vector::<Vector<Point2f>>::new();

        aruco::detect_markers(
            &img.mat,
            &self.dictionary,
            &mut corners,
            &mut ids,
            &self.parameters,
            &mut rejected,
        ).unwrap();

        HashMap::from_iter(ids.iter().zip(corners.iter()))
    }

    pub fn read_image(&self, path: &str) -> Img {
        let mat = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR);
        Img { mat: mat.unwrap() }
    }

}
