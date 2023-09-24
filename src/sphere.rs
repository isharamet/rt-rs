use crate::hitable::{HitRecord, Hitable};
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;

        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            let mut t = (-half_b - sqrt_d) / a;

            if t <= t_min || t >= t_max {
                t = (-half_b + sqrt_d) / a;

                if t > t_min && t < t_max {
                    let p = ray.point_at(t);

                    result = Some(HitRecord {
                        point: p,
                        normal: (p - self.center) / self.radius,
                        t,
                    });
                }
            } else {
                let p = ray.point_at(t);

                result = Some(HitRecord {
                    point: p,
                    normal: (p - self.center) / self.radius,
                    t,
                });
            }
        }

        result
    }
}
