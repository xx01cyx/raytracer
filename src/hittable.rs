use crate::vec3::{Vec3, Point3, Color};
use crate::ray::Ray;
use crate::{Material, Lambertian, Metal};
use crate::aabb::{self, AABB};
use crate::texture::{Texture, SolidColor, CheckerTexture};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,  // have the normals always point against the ray
}

impl HitRecord {
    pub fn new(m: Arc<dyn Material>) -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat_ptr: m,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = ((r.direction * outward_normal)) < 0.0;
        self.normal =  if self.front_face { outward_normal } else { -outward_normal };
    }
}



pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, output_box: &mut AABB) -> bool;
}


// Hittable list

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::zero())))));
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        
        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        if self.objects.len() == 0 {
            return false;
        }

        let mut temp_box = AABB::new(Point3::zero(),Point3::zero());
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(&mut temp_box) {
                return false;
            }
            *output_box = if first_box { temp_box.clone() } else { aabb::surrounding_box(&output_box, &temp_box) };
            first_box = false;
        }

        return true;
    } 
}




