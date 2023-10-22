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
    pub defocus_angle: f32,
    pub focus_dist: f32,
    img_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

pub struct CameraBuilder {
    pub aspect_ratio: f32,
    pub img_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub fov: u32,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            aspect_ratio: 1.0,
            img_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            fov: 90,
            lookfrom: Vec3::new(0.0, 0.0, -1.0),
            lookat: Vec3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f32) -> CameraBuilder {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn img_width(mut self, img_width: u32) -> CameraBuilder {
        self.img_width = img_width;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> CameraBuilder {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> CameraBuilder {
        self.max_depth = max_depth;
        self
    }

    pub fn fov(mut self, fov: u32) -> CameraBuilder {
        self.fov = fov;
        self
    }

    pub fn lookfrom(mut self, lookfrom: Vec3) -> CameraBuilder {
        self.lookfrom = lookfrom;
        self
    }

    pub fn lookat(mut self, lookat: Vec3) -> CameraBuilder {
        self.lookat = lookat;
        self
    }

    pub fn vup(mut self, vup: Vec3) -> CameraBuilder {
        self.vup = vup;
        self
    }

    pub fn defocus_angle(mut self, defocus_angle: f32) -> CameraBuilder {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn focus_dist(mut self, focus_dist: f32) -> CameraBuilder {
        self.focus_dist = focus_dist;
        self
    }

    pub fn build(&self) -> Camera {
        let img_height: u32 = (self.img_width as f32 / self.aspect_ratio) as u32;

        let center = self.lookfrom;

        let theta = (self.fov as f32).to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h * self.focus_dist;
        let viewport_width: f32 = viewport_height * (self.img_width as f32 / img_height as f32);

        let w = (self.lookfrom - self.lookat).unit_vector();
        let u = self.vup.cross(w).unit_vector();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / self.img_width as f32;
        let pixel_delta_v = viewport_v / img_height as f32;

        let viewport_upper_left =
            center - (self.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_u);

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            aspect_ratio: self.aspect_ratio,
            img_width: self.img_width,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            fov: self.fov,
            lookfrom: self.lookat,
            lookat: self.lookat,
            vup: self.vup,
            defocus_angle: self.defocus_angle,
            focus_dist: self.focus_dist,
            img_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

fn linear_to_gamma(linear: f32) -> f32 {
    linear.sqrt()
}

impl Camera {
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

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
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

        let ray_origin = if self.defocus_angle < 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
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
