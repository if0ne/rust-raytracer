mod vec3;
mod ray;
mod hittable;
mod sphere;
mod world;
mod camera;

use indicatif::ProgressBar;
use std::path::Path;
use std::rc::Rc;
use rand::Rng;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};
use crate::world::World;

fn ray_color(r: &Ray, entity: &dyn Hittable) -> Color {
    if let Some(record) = entity.hit(r, 0.0, f64::INFINITY) {
        return 0.5*(record.normal + Color::with(1.0, 1.0, 1.0));
    }

    let unit = r.direction().normalize();
    let t = 0.5 * (unit.y() + 1.0);

    (1.0 - t)*Color::with(1.0, 1.0, 1.0) + t*Color::with(0.5, 0.7, 1.0)
}

fn main() {

    //Image
    let aspect_ration = 16.0 / 9.0;
    let width = 400u32;
    let height = (width as f64 / aspect_ration).trunc() as u32;
    let samples_per_pixel = 100;

    //World
    let mut world = World::new();
    world.add(Rc::new(Sphere::new(&Point3::with(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(&Point3::with(0.0, -100.5, -1.0), 100.0)));

    //Camera
    let camera = Camera::new();

    //Render
    let mut img_buf = image::ImageBuffer::new(width, height);
    let bar = ProgressBar::new((width * height * samples_per_pixel) as u64);

    for (i, j, pixel) in img_buf.enumerate_pixels_mut() {
        let mut pixel_color = Color::new();
        for _ in 0..samples_per_pixel {
            let u = (i as f64 + rand::thread_rng().gen::<f64>()) / (width - 1) as f64;
            let v = ((height - j) as f64 + rand::thread_rng().gen::<f64>()) / (height - 1) as f64;
            let r = camera.get_ray(u, v);
            pixel_color += ray_color(&r, &world);

            *pixel = image::Rgb(pixel_color.into_rgb(samples_per_pixel as f64));

            bar.inc(1);
        }
    }

    img_buf.save(Path::new("image.png")).unwrap();

    bar.finish();
}
