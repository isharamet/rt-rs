use rt_rs::camera::Camera;
use rt_rs::hittable::Hittable;
use rt_rs::material::{Dielectric, Lambertian, Material, Metal};
use rt_rs::sphere::Sphere;
use rt_rs::vec3::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let fov = 120;
    let camera = Camera::new(aspect_ratio, img_width, samples_per_pixel, max_depth, fov);

    let m_ground = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let m_center = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    let m_left = Material::Dielectric(Dielectric { ir: 1.5 });
    let m_right = Material::Metal(Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, m_center)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, m_ground)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, m_left)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, m_left)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, m_right)),
    ];

    camera.render(&world);
}
