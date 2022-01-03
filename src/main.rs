mod vec3;
mod ray;
mod hittable;
mod sphere;
mod world;

use indicatif::ProgressBar;
use std::path::Path;
use std::rc::Rc;
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

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
    //B*B*t^2 + 2*B*(A-C)*t + (A - C)*(A - C) - r^2 = 0
    let oc = r.origin() - *center;                          //A - C
    let a = r.direction().len_sqr();                         //B*B (coef: a)
    let b = Vec3::dot(&(r.direction()), &oc);         //2*B*(A - C) (coef: b)
    let c = oc.len_sqr() - radius * radius;                 //(A - C)*(A - C) - r^2 (coef: c)

    let discriminant = b*b - a*c;

    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (a))
    }
}

fn main() {

    //Image
    let aspect_ration = 16.0 / 9.0;
    let width = 400u32;
    let height = (width as f64 / aspect_ration).trunc() as u32;

    //World
    let mut world = World::new();
    world.add(Rc::new(Sphere::new(&Point3::with(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(&Point3::with(0.0, -100.5, -1.0), 100.0)));

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ration * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new();
    let horizontal = Vec3::with(viewport_width, 0.0,0.0);
    let vertical = Vec3::with(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::with(0.0, 0.0, focal_length);

    //Render
    let mut img_buf = image::ImageBuffer::new(width, height);
    let bar = ProgressBar::new((width * height) as u64);

    for (i, j, pixel) in img_buf.enumerate_pixels_mut() {
        let u = (i as f64) / (width - 1) as f64;
        let v = ((height - j) as f64) / (height - 1) as f64;
        let r = Ray::with(
            &origin,
            &(lower_left_corner + u*horizontal + v*vertical - origin)
        );

        let mut pixel_color = ray_color(&r, &world);
        *pixel = image::Rgb(pixel_color.into());

        bar.inc(1);
    }

    img_buf.save(Path::new("image.png")).unwrap();

    bar.finish();
}
