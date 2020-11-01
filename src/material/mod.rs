mod lambertian;
mod metal;

use std::fmt::Debug;
use std::marker::{Send, Sync};

use crate::{color::Color, hit_record::HitRecord, ray::Ray};

pub trait Material: Debug + Sync + Send {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<MaterialResult>;
}

#[derive(Debug, Copy, Clone)]
pub struct MaterialResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub use lambertian::Lambertian;
pub use metal::Metal;
