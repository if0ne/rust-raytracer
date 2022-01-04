use rand::Rng;
use crate::hittable::HitRecord;
use crate::{Color, Ray, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub color: Color,
}

impl Lambertian {
    pub fn new(color: &Color) -> Self {
        Self {
            color: *color
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = {
            let dir = record.normal + Vec3::random_unit_vector();
            if dir.near_zero() {
                record.normal
            } else {
                dir
            }
        };

        let scattered = Ray::with(&record.point, &scatter_direction);
        Some((scattered, self.color))
    }
}

pub struct Metal {
    pub color: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(color: &Color, fuzz: f64) -> Self {
        Self {
            color: *color,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(&ray.direction().normalize(), &record.normal);
        let scattered = Ray::with(&record.point, &(reflected + self.fuzz * Vec3::random_in_unit_sphere()));

        if Vec3::dot(&scattered.direction(), &record.normal) > 0.0 {
            Some((scattered, self.color))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self {
            ir
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::with(1.0, 1.0, 1.0);
        let refraction_ratio = if record.front_face { (1.0/self.ir) } else { self.ir };
        let unit_dir = ray.direction().normalize();

        let cos_theta =  {
            let dot = Vec3::dot(&-unit_dir, &record.normal);
            if dot > 1.0 {
                1.0
            } else {
                dot
            }
        };
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let dir = if refraction_ratio * sin_theta > 1.0 ||
            reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f64>() {
            //Отражение
            Vec3::reflect(&unit_dir, &record.normal)
        } else {
            //Преломление
            Vec3::refract(&unit_dir, &record.normal, refraction_ratio)
        };

        let scattered = Ray::with(&record.point, &dir);
        Some((scattered, attenuation))
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0)*(1.0 - cosine).powf(5.0)
}