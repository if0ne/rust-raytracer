use crate::vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point3::new(),
            dir: Vec3::new(),
        }
    }

    pub fn with(origin: &Point3, dir: &Vec3) -> Self {
        Self {
            origin: *origin,
            dir: *dir,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}