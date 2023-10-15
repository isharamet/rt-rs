use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::material::{Lambertian, Material, Metal};
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod rng;
mod sphere;
mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let camera = Camera::new(aspect_ratio, img_width, samples_per_pixel, max_depth);

    let m_ground = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let m_center = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    });
    let m_left = Material::Metal(Metal {
        albedo: Vec3::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    });
    let m_right = Material::Metal(Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, m_center)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, m_ground)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, m_left)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, m_right)),
    ];

    camera.render(&world);
}
