use crate::hittable::HitRecord;
use crate::{Hittable, Ray};
use std::rc::Rc;

pub struct World {
    entities: Vec<Rc<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        World { entities: vec![] }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.entities.push(object);
    }

    pub fn clear(&mut self) {
        self.entities.clear()
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        //Переписать на немутабельность
        let mut closest_so_far = t_max;
        let mut res = None;

        for entity in self.entities.iter() {
            if let Some(record) = entity.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                res = Some(record);
            }
        }

        res
    }
}