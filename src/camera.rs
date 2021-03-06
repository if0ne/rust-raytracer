use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        vert_fov: f64,
        aspect_ratio: f64,
        look_from: &Point3,
        look_at: &Point3,
        vup: &Vec3,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vert_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (*look_from - *look_at).normalize();
        let u = Vec3::cross(vup, &w).normalize();
        let v = Vec3::cross(&w, &u);

        let focal_length = 1.0;

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = *origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            origin: *origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = s * rd.x() + t * rd.y();

        Ray::with(
            &(self.origin + offset),
            &(self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin
                + (-offset)),
        )
    }
}