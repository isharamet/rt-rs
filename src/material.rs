use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rng;
use crate::vec3::Vec3;

pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<Scatter>;
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<Scatter> {
        match self {
            Material::Lambertian(l) => l.scatter(ray_in, hit_rec),
            Material::Metal(m) => m.scatter(ray_in, hit_rec),
            Material::Dielectric(d) => d.scatter(ray_in, hit_rec),
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
        let scattered = Ray::new(hit_rec.point, direction);

        Some(Scatter {
            attenuation: self.albedo,
            ray: scattered,
        })
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<Scatter> {
        let direction = Vec3::reflect(ray_in.direction.unit_vector(), hit_rec.normal);
        let reflected = Ray::new(
            hit_rec.point,
            direction + self.fuzz * Vec3::random_unit_vector(),
        );

        Some(Scatter {
            attenuation: self.albedo,
            ray: reflected,
        })
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub ir: f32,
}

impl Dielectric {
    pub fn reflectance(cos: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f32::powi(1.0 - cos, 5)
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if hit_rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = f32::min((-unit_direction).dot(hit_rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng::random()
        {
            Vec3::reflect(unit_direction, hit_rec.normal)
        } else {
            Vec3::refract(unit_direction, hit_rec.normal, refraction_ratio)
        };
        let refracted = Ray::new(hit_rec.point, direction);

        Some(Scatter {
            attenuation: Vec3::new(1.0, 1.0, 1.0),
            ray: refracted,
        })
    }
}
