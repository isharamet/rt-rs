use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    fn hit_record(&self, ray: &Ray, t: f64) -> HitRecord {
        let point = ray.point_at(t);
        let outward_normal = (point - self.center) / self.radius;
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;

        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            let mut t = (-half_b - sqrt_d) / a;

            if !ray_t.surrounds(t) {
                t = (-half_b + sqrt_d) / a;
                if ray_t.surrounds(t) {
                    result = Some(self.hit_record(ray, t));
                }
            } else {
                result = Some(self.hit_record(ray, t));
            }
        }

        result
    }
}
