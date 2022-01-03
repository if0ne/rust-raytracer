use crate::{Point3, Ray, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(ray: &Ray, center: &Point3, root: f64, radius: f64) -> Self {
        let t = root;
        let point = ray.at(t);
        let (normal, front_face) = {
            let outward_normal = (point - * center) / radius;
            let front_face = Vec3::dot(&ray.direction(), &outward_normal) < 0.0;
            let normal = if front_face { outward_normal } else { -outward_normal };

            (normal, front_face)
        };
        
        HitRecord {
            point,
            normal,
            t,
            front_face
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}