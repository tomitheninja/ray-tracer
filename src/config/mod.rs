mod from_args;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub img_width: usize,
    pub img_height: usize,
    pub samples_per_pixel: usize,
    pub max_ray_depth: usize,
    pub output_file: String,
}

impl Config {
    pub fn aspect_ratio(&self) -> f64 {
        self.img_width as f64 / self.img_height as f64
    }
}
