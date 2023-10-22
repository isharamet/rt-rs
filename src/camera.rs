use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::rng;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub struct Camera {
    pub aspect_ratio: f32,
    pub img_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub fov: u32,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    img_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

fn linear_to_gamma(linear: f32) -> f32 {
    linear.sqrt()
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        img_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        fov: u32,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
    ) -> Camera {
        let img_height: u32 = (img_width as f32 / aspect_ratio) as u32;

        let center = lookfrom;

        let focal_length: f32 = (lookfrom - lookat).length();
        let theta = (fov as f32).to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h * focal_length;
        let viewport_width: f32 = viewport_height * (img_width as f32 / img_height as f32);

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / img_width as f32;
        let pixel_delta_v = viewport_v / img_height as f32;

        let viewport_upper_left = center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_u);

        Camera {
            aspect_ratio,
            img_width,
            samples_per_pixel,
            img_height,
            max_depth,
            fov,
            lookfrom,
            lookat,
            vup,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
        }
    }

    fn ray_color(ray: &Ray, depth: u32, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
        if depth <= 0 {
            Vec3::zero()
        } else {
            match world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
                Some(hit_rec) => match hit_rec.material.scatter(&ray, &hit_rec) {
                    Some(scattered) => {
                        scattered.attenuation * Camera::ray_color(&scattered.ray, depth - 1, world)
                    }
                    None => Vec3::zero(),
                },
                None => {
                    let unit_direction = ray.direction.unit_vector();
                    let a: f32 = 0.5 * (unit_direction.y() + 1.0);

                    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
                }
            }
        }
    }

    fn write_color(&self, mut file: &File, color: Vec3) {
        let scale = 1.0 / self.samples_per_pixel as f32;

        let r = linear_to_gamma(color.x() * scale);
        let g = linear_to_gamma(color.y() * scale);
        let b = linear_to_gamma(color.z() * scale);

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
            self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
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
                    pixel_color = pixel_color + Self::ray_color(&ray, self.max_depth, &world);
                }

                self.write_color(&file, pixel_color);
            }
        }
    }
}
