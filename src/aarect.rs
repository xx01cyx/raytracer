use crate::vec3::{Vec3, Point3, Color};
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::aabb::AABB;
use std::sync::Arc;


// X-Y Rectangle

pub struct XyRect {
    pub mp: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self { mp, x0, x1, y0, y1, k }
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t: f64 = (self.k - r.origin.z) / r.direction.z;
        if t < t_min || t > t_max {
            return false;
        }

        let x: f64 = r.origin.x + r.direction.x * t;
        let y: f64 = r.origin.y + r.direction.y * t;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);

        return true;
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(Point3::new(self.x0, self.y0, self.k-0.0001), Point3::new(self.x1, self.y1, self.k+0.0001));
        
        return true;
    }
}


// X-Z Rectangle

pub struct XzRect {
    pub mp: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self { mp, x0, x1, z0, z1, k }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t: f64 = (self.k - r.origin.y) / r.direction.y;
        if t < t_min || t > t_max {
            return false;
        }

        let x: f64 = r.origin.x + r.direction.x * t;
        let z: f64 = r.origin.z + r.direction.z * t;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, -1.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);

        return true;
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(Point3::new(self.x0, self.k-0.0001, self.z0), Point3::new(self.x1, self.k+0.0001, self.z1));
        
        return true;
    }
}


// Y-Z Rectangle

pub struct YzRect {
    pub mp: Arc<dyn Material>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self { mp, y0, y1, z0, z1, k }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t: f64 = (self.k - r.origin.x) / r.direction.x;
        if t < t_min || t > t_max {
            return false;
        }

        let y: f64 = r.origin.y + r.direction.y * t;
        let z: f64 = r.origin.z + r.direction.z * t;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);

        return true;
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(Point3::new(self.k-0.0001, self.y0, self.z0), Point3::new(self.k+0.0001, self.y1, self.z1));
        
        return true;
    }
}