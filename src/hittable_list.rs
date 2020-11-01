use super::{HitRecord, Hittable};
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    // pub fn new(obj: Rc<dyn Hittable>) -> Self {
    //     HittableList { objects: vec![obj] }
    // }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj)
    }

    pub fn chain_add(mut self, obj: Rc<dyn Hittable>) -> Self {
        self.add(obj);
        self
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Find the object that hits the ray first
        self.objects.iter().fold(None, |best, x| {
            let closest_so_far = best.as_ref().map_or(t_max, |x| x.t);
            x.hit(r, t_min, closest_so_far).or(best)
        })
    }
}
