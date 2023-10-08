use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod camera;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 400;
    let samples_per_pixel = 100;
    let camera = Camera::new(aspect_ratio, img_width, samples_per_pixel);

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];

    camera.render(&world);
}
