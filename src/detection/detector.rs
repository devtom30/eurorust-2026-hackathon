use opencv::{
    aruco,
    core::{Mat, Point2f, Ptr, Vector},
    imgcodecs,
    prelude::*,
};

pub struct Detector {
    dictionary: Ptr<aruco::Dictionary>,
    parameters: Ptr<aruco::DetectorParameters>,
}

pub struct Img {
    mat: Mat,
}

impl Img {
    /// Decode an encoded image (PNG, JPEG, ...) from bytes, e.g. a reqwest response body.
    pub fn from_bytes(bytes: &[u8]) -> opencv::Result<Self> {
        let buf = Vector::<u8>::from_slice(bytes);
        let mat = imgcodecs::imdecode(&buf, imgcodecs::IMREAD_COLOR)?;
        Ok(Self { mat })
    }
}

impl Detector {

    pub fn new() -> Self {
        let dictionary =
            aruco::get_predefined_dictionary(aruco::PREDEFINED_DICTIONARY_NAME::DICT_4X4_50).unwrap();
        let parameters = aruco::DetectorParameters::create().unwrap();

        Self { dictionary, parameters }
    }

    pub fn detect(&self, img: &Img) -> Vec<(i32, Vector<Point2f>)> {
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

        ids.iter().zip(corners.iter()).collect()
    }

    pub fn read_image(&self, path: &str) -> Img {
        let mat = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR);
        Img { mat: mat.unwrap() }
    }

}
