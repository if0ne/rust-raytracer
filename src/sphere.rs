use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::{Point3, Ray, Vec3};
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center: *center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        //B*B*t^2 + 2*B*(A-C)*t + (A - C)*(A - C) - r^2 = 0
        //Формула корней квадратного уравнения с четным вторым коэффициентом
        let oc = ray.origin() - self.center; //A - C
        let a = ray.direction().len_sqr(); //B*B (coef: a)
        let b = Vec3::dot(&(ray.direction()), &oc); //2*B*(A - C) (coef: b)
        let c = oc.len_sqr() - self.radius * self.radius; //(A - C)*(A - C) - r^2 (coef: c)

        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        //Переписать на немутабельность
        let sqrtd = discriminant.sqrt();
        let mut root = (-b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-b + sqrtd) / a;
        };

        if root < t_min || t_max < root {
            return None;
        };

        Some(HitRecord::new(
            ray,
            &self.center,
            root,
            self.radius,
            self.material.clone(),
        ))
    }
}