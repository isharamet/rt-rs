use hittable::Hittable;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

mod hittable;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
    match world.hit(ray, Interval::new(0.0, f32::INFINITY)) {
        Some(hit_rec) => 0.5 * (hit_rec.normal + Vec3::new(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = ray.direction.unit_vector();
            let a: f32 = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn color_str(color: Vec3) -> String {
    format!(
        "{} {} {}\n",
        (255.999 * color.x()) as u32,
        (255.999 * color.y()) as u32,
        (255.999 * color.z()) as u32
    )
}

fn main() {
    // Image
    let img_format = "P3";

    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 400;
    let img_height: u32 = (img_width as f32 / aspect_ratio) as u32;

    // World
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

    // Camera
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (img_width as f32 / img_height as f32);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_u = viewport_u / (img_width as f32);
    let pixel_v = viewport_v / (img_height as f32);

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_u + pixel_v);

    let max_colors: u32 = 255;

    let mut file = File::create("img.ppm").expect("Unable to create file");

    write!(
        file,
        "{}\n{} {}\n{}\n",
        img_format, img_width, img_height, max_colors
    )
    .expect("Unable to write to file");

    for j in 0..img_height {
        print!("\rScanlines remaining: {}", img_height - j);
        for i in 0..img_width {
            let pixel_center = pixel00_loc + (i as f32 * pixel_u) + (j as f32 * pixel_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = ray_color(&ray, &world);

            file.write(color_str(color).as_bytes())
                .expect("Unable to write to file");
        }
    }
}
