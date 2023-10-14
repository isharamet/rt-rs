use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl Hittable for HitRecord {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        None
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        let mut closest = ray_t.max;

        for hittable in self.iter() {
            if let Some(curr_hit) = hittable.hit(ray, Interval::new(ray_t.min, closest)) {
                match hit {
                    None => hit = Some(curr_hit),
                    Some(prev_hit) => {
                        closest = prev_hit.t;
                        if curr_hit.t < prev_hit.t {
                            hit = Some(curr_hit);
                        }
                    }
                }
            }
        }

        hit
    }
}
