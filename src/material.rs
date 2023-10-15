use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<Scatter>;
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<Scatter> {
        match self {
            Material::Lambertian(l) => l.scatter(ray_in, hit_rec),
            Material::Metal(m) => m.scatter(ray_in, hit_rec),
        }
    }
}
#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Scatterable for Lambertian {
    fn scatter(&self, _: &Ray, hit_rec: &HitRecord) -> Option<Scatter> {
        let mut direction = hit_rec.normal + Vec3::random_unit_vector();
        if direction.near_zero() {
            direction = hit_rec.normal
        }
        let ray = Ray::new(hit_rec.point, direction);
        Some(Scatter {
            attenuation: self.albedo,
            ray,
        })
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<Scatter> {
        let direction = Vec3::reflect(ray_in.direction.unit_vector(), hit_rec.normal);
        let ray = Ray::new(hit_rec.point, direction);
        Some(Scatter {
            attenuation: self.albedo,
            ray,
        })
    }
}
