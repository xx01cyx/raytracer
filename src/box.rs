use crate::vec3::{Vec3, Color, Point3};
use crate::ray::Ray;
use crate::aarect::{XyRect, YzRect, XzRect};
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::material::Material;
use crate::aabb::AABB;
use crate::utils::*;
use std::sync::Arc;

pub struct Box {
    pub box_min: Point3,
    pub box_max: Point3,
    pub sides: HittableList
}

impl Box {
    pub fn new(p0: Point3, p1: Point3, ptr: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        sides.add(Arc::new(XyRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, ptr.clone())));
        sides.add(Arc::new(XyRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, ptr.clone())));
        sides.add(Arc::new(XzRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, ptr.clone())));
        sides.add(Arc::new(XzRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, ptr.clone())));
        sides.add(Arc::new(YzRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, ptr.clone())));
        sides.add(Arc::new(YzRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, ptr.clone())));

        Self {
            box_min: p0,
            box_max: p1,
            sides: sides
        }
    }
}

impl Hittable for Box {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.box_min, self.box_max);

        return true;
    }
}