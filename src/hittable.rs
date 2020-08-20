use crate::aabb::{self, AABB};
use crate::ray::Ray;
use crate::texture::SolidColor;
use crate::utils::*;
use crate::vec3::{Color, Point3, Vec3};
use crate::{Lambertian, Material};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool, // have the normals always point against the ray
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
        self.front_face = (r.direction * outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, output_box: &mut AABB) -> bool;
}

// Hittable list

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(Arc::new(Lambertian::new(Arc::new(SolidColor::new(
            Color::zero(),
        )))));
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = AABB::new(Point3::zero(), Point3::zero());
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(&mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box.clone()
            } else {
                aabb::surrounding_box(&output_box, &temp_box)
            };
            first_box = false;
        }

        true
    }
}

// Rotate-y

pub struct RotateY {
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: AABB,
}

impl RotateY {
    pub fn new(ptr: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians: f64 = degrees_to_radians(angle);
        let sin_theta: f64 = radians.sin();
        let cos_theta: f64 = radians.cos();

        let mut bbox: AABB = AABB::new(Point3::zero(), Point3::zero());
        let hasbox: bool = ptr.bounding_box(&mut bbox);

        let mut min: Point3 = Point3::new(INF, INF, INF);
        let mut max: Point3 = Point3::new(-INF, -INF, -INF);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x: f64 = i as f64 * bbox._max.x + (1 - i) as f64 * bbox._min.x;
                    let y: f64 = j as f64 * bbox._max.y + (1 - j) as f64 * bbox._min.y;
                    let z: f64 = k as f64 * bbox._max.z + (1 - k) as f64 * bbox._min.z;

                    let newx: f64 = cos_theta * x + sin_theta * z;
                    let newz: f64 = -sin_theta * x + cos_theta * z;

                    let tester: Vec3 = Vec3::new(newx, y, newz);

                    min.x = fmin(min.x, tester.x);
                    max.x = fmin(max.x, tester.x);
                    min.y = fmin(min.y, tester.y);
                    max.y = fmin(max.y, tester.y);
                    min.z = fmin(min.z, tester.z);
                    max.z = fmin(max.z, tester.z);
                }
            }
        }

        bbox = AABB::new(min, max);

        Self {
            ptr,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin: Point3 = r.origin;
        let mut direction: Vec3 = r.direction;

        origin.x = r.origin.x * self.cos_theta - r.origin.z * self.sin_theta;
        origin.z = r.origin.x * self.sin_theta + r.origin.z * self.cos_theta;

        direction.x = r.direction.x * self.cos_theta - r.direction.z * self.sin_theta;
        direction.z = r.direction.x * self.sin_theta + r.direction.z * self.cos_theta;

        let rotated_r: Ray = Ray::new(origin, direction);
        if !self.ptr.hit(rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p: Point3 = rec.p;
        let normal: Vec3 = rec.normal;

        p.x = rec.normal.x * self.cos_theta + rec.normal.z * self.sin_theta;
        p.z = rec.normal.x * (-self.sin_theta) + rec.normal.z * self.cos_theta;

        rec.p = p;
        rec.set_face_normal(rotated_r, normal);

        true
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        *output_box = self.bbox.clone();

        self.hasbox
    }
}
