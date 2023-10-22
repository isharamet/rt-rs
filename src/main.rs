use rt_rs::camera::CameraBuilder;
use rt_rs::hittable::Hittable;
use rt_rs::material::{Dielectric, Lambertian, Material, Metal};
use rt_rs::rng;
use rt_rs::sphere::Sphere;
use rt_rs::vec3::Vec3;

fn main() {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let camera = CameraBuilder::new()
        .aspect_ratio(16.0 / 9.0)
        .img_width(1200)
        .samples_per_pixel(500)
        .max_depth(50)
        .fov(20)
        .lookfrom(Vec3::new(13.0, 2.0, 3.0))
        .lookat(Vec3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .build();

    let ground_material = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng::random();
            let center = Vec3::new(
                a as f32 + 0.9 * rng::random(),
                0.2,
                b as f32 + 0.9 * rng::random(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Vec3::random();
                    let material = Material::Lambertian(Lambertian { albedo });
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else if choose_material < 0.95 {
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = rng::random_in_range(0.0, 0.5);
                    let material = Material::Metal(Metal { albedo, fuzz });
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Material::Dielectric(Dielectric { ir: 1.5 });
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Material::Dielectric(Dielectric { ir: 1.5 });
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    });
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::Metal(Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    camera.render(&world);
}
