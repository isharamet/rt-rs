use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub struct Camera {
    pub aspect_ratio: f32,
    pub img_width: u32,
    img_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

fn color_str(color: Vec3) -> String {
    format!(
        "{} {} {}\n",
        (255.999 * color.x()) as u32,
        (255.999 * color.y()) as u32,
        (255.999 * color.z()) as u32
    )
}

pub fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
    match world.hit(ray, Interval::new(0.0, f32::INFINITY)) {
        Some(hit_rec) => 0.5 * (hit_rec.normal + Vec3::new(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = ray.direction.unit_vector();
            let a: f32 = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

impl Camera {
    pub fn new(aspect_ratio: f32, img_width: u32) -> Camera {
        let img_height: u32 = (img_width as f32 / aspect_ratio) as u32;

        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (img_width as f32 / img_height as f32);
        let center = Vec3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (img_width as f32);
        let pixel_delta_v = viewport_v / (img_height as f32);

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_u);

        Camera {
            aspect_ratio,
            img_width,
            img_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &Vec<Box<dyn Hittable>>) {
        let img_format = "P3";
        let max_colors: u32 = 255;

        let mut file = File::create("img.ppm").expect("Unable to create file");

        write!(
            file,
            "{}\n{} {}\n{}\n",
            img_format, self.img_width, self.img_height, max_colors
        )
        .expect("Unable to write to file");

        for j in 0..self.img_height {
            print!("\rScanlines remaining: {}", self.img_height - j);
            for i in 0..self.img_width {
                let pixel_center = self.pixel00_loc
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                let color = ray_color(&ray, &world);

                file.write(color_str(color).as_bytes())
                    .expect("Unable to write to file");
            }
        }
    }
}
