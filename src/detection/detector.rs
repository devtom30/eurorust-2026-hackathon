use opencv::{
    aruco,
    core::{Mat, Point2f, Ptr, Size, Vector},
    imgcodecs, imgproc,
    prelude::*,
};
use std::{collections::HashMap};

const UPSCALE: f32 = 2.0;

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
        let mut parameters = aruco::DetectorParameters::create().unwrap();
        // Accept smaller markers than the default (0.03)
        parameters.set_min_marker_perimeter_rate(0.02);
        // Markers are seen at a steep angle, so allow less-perfect quads.
        parameters.set_polygonal_approx_accuracy_rate(0.05);
        // Sample more pixels per bit and ignore cell edges, which blur on small markers.
        parameters.set_perspective_remove_pixel_per_cell(8);
        parameters.set_perspective_remove_ignored_margin_per_cell(0.2);
        parameters.set_corner_refinement_method(aruco::CORNER_REFINE_SUBPIX);

        Self { dictionary, parameters }
    }

    pub fn detect(&self, img: &Img) -> HashMap<i32, Vector<Point2f>> {
        let mut gray = Mat::default();  // improve detection with gray scale
        imgproc::cvt_color(&img.mat, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();

        let mut equalized = Mat::default();
        imgproc::create_clahe(2.0, Size::new(8, 8)).unwrap()  // make contrast more obvious to
            // simplify detection
            .apply(&gray, &mut equalized).unwrap();
        let mut upscaled = Mat::default();
        imgproc::resize(  // upscale because I'm not sure how to make it detect smaller tags
            &equalized,
            &mut upscaled,
            Size::new(0, 0),
            UPSCALE as f64,
            UPSCALE as f64,
            imgproc::INTER_CUBIC,
        ).unwrap();

        let mut found = self.detect_markers(&upscaled, UPSCALE);  // detect upscaled and contrast image
        for (id, corners) in self.detect_markers(&gray, 1.0) {  // up fill original image
           found.entry(id).or_insert(corners);
        }
        found
    }

    fn detect_markers(&self, mat: &Mat, scale: f32) -> HashMap<i32, Vector<Point2f>> {
        let mut corners = Vector::<Vector<Point2f>>::new();
        let mut ids = Vector::<i32>::new();
        let mut rejected = Vector::<Vector<Point2f>>::new();

        aruco::detect_markers(
            mat,
            &self.dictionary,
            &mut corners,
            &mut ids,
            &self.parameters,
            &mut rejected,
        ).unwrap();

        ids.iter()
            .zip(corners.iter())
            .map(|(id, c)| (id, c.iter().map(|p| Point2f::new(p.x / scale, p.y / scale)).collect()))
            .collect()
    }

    pub fn read_image(&self, path: &str) -> Img {
        let mat = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR);
        Img { mat: mat.unwrap() }
    }

}
