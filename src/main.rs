mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod world;

use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::*;
use std::path::Path;
use std::rc::Rc;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};
use crate::world::World;

fn ray_color(r: &Ray, entity: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::with(0.0, 0.0, 0.0);
    }

    if let Some(record) = entity.hit(r, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = record.material.scatter(r, &record) {
            return attenuation * ray_color(&scattered, entity, depth - 1);
        } else {
            return Color::new();
        }
    }

    let unit = r.direction().normalize();
    let t = 0.5 * (unit.y() + 1.0);

    (1.0 - t) * Color::with(1.0, 1.0, 1.0) + t * Color::with(0.5, 0.7, 1.0)
}

fn main() {
    //Image
    let aspect_ration = 16.0 / 9.0;
    let width = 400u32;
    let height = (width as f64 / aspect_ration).trunc() as u32;
    let samples_per_pixel = 10;
    let max_depth = 50;

    //World
    let mat_ground = Rc::new(Lambertian::new(&Color::with(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(&Color::with(0.7, 0.3, 0.3)));
    let mat_left = Rc::new(Metal::new(&Color::with(0.8, 0.8, 0.3), 0.5));
    let mat_right = Rc::new(Dielectric::new(1.5));

    let mut world = World::new();
    world.add(Rc::new(Sphere::new(
        &Point3::with(0.0, -100.5, -1.0),
        100.0,
        mat_ground.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::with(0.0, 0.0, -1.0),
        0.5,
        mat_center.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::with(-1.0, 0.0, -1.0),
        0.5,
        mat_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::with(1.0, 0.0, -1.0),
        0.5,
        mat_right.clone(),
    )));

    //Camera
    let camera = Camera::new(
        20.0,
        aspect_ration,
        &Point3::with(3.0, 3.0, 2.0),
        &Point3::with(0.0, 0.0, -1.0),
        &Vec3::with(0.0, 1.0, 0.0),
        2.0,
        (Point3::with(3.0, 3.0, 2.0) - Point3::with(0.0, 0.0, -1.0)).len(),
    );

    //Render
    let mut img_buf = image::ImageBuffer::new(width, height);
    let bar = ProgressBar::new((width * height * samples_per_pixel) as u64);

    for (i, j, pixel) in img_buf.enumerate_pixels_mut() {
        let mut pixel_color = Color::new();
        for _ in 0..samples_per_pixel {
            let u = (i as f64 + rand::thread_rng().gen::<f64>()) / (width - 1) as f64;
            let v = ((height - j) as f64 + rand::thread_rng().gen::<f64>()) / (height - 1) as f64;
            let r = camera.get_ray(u, v);
            pixel_color += ray_color(&r, &world, max_depth);

            *pixel = image::Rgb(pixel_color.into_rgb(samples_per_pixel as f64));

            bar.inc(1);
        }
    }

    img_buf.save(Path::new("image.png")).unwrap();

    bar.finish();
}