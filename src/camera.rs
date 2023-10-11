use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rng;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: u32,
    pub samples_per_pixel: u32,
    img_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, img_width: u32, samples_per_pixel: u32) -> Camera {
        let img_height: u32 = (img_width as f64 / aspect_ratio) as u32;

        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (img_width as f64 / img_height as f64);
        let center = Vec3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (img_width as f64);
        let pixel_delta_v = viewport_v / (img_height as f64);

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_u);

        Camera {
            aspect_ratio,
            img_width,
            samples_per_pixel,
            img_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
        match world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            Some(hit_rec) => {
                let direction = Vec3::random_on_hemisphere(hit_rec.normal);
                0.5 * Camera::ray_color(&Ray::new(hit_rec.point, direction), world)
            }
            None => {
                let unit_direction = ray.direction.unit_vector();
                let a: f64 = 0.5 * (unit_direction.y() + 1.0);

                (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn write_color(&self, mut file: &File, color: Vec3) {
        let scale = 1.0 / self.samples_per_pixel as f64;

        let r = color.x() * scale;
        let g = color.y() * scale;
        let b = color.z() * scale;

        let intensity = Interval::new(0.0, 0.999);

        let color_str = format!(
            "{} {} {}\n",
            (256.0 * intensity.clamp(r)) as u32,
            (256.0 * intensity.clamp(g)) as u32,
            (256.0 * intensity.clamp(b)) as u32
        );

        file.write(color_str.as_bytes())
            .expect("Unable to write to file");
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rng::random();
        let py = -0.5 + rng::random();
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
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
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color = pixel_color + Self::ray_color(&ray, &world);
                }

                self.write_color(&file, pixel_color);
            }
        }
    }
}
