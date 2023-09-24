use crate::ray::Ray;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

mod ray;
mod vec3;

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(ray: &Ray) -> Vec3 {
    let c = Vec3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(c, 0.5, ray);
    if t > 0.0 {
        let n = (ray.point_at(t) - c).unit_vector();
        0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
    } else {
        let unit_direction = ray.direction.unit_vector();
        let a: f32 = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
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
    let img_format = "P3";

    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 400;
    let img_height: u32 = (img_width as f32 / aspect_ratio) as u32;

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
            let color = ray_color(&ray);

            file.write(color_str(color).as_bytes())
                .expect("Unable to write to file");
        }
    }
}
