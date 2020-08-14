use crate::vec3::{ Vec3, Point3, Color };
use crate::ray::Ray;
use crate::hittable::{ HitRecord, Hittable };
use crate::material::Material;
use std::sync::Arc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center: cen,
            radius: r,
            mat_ptr: m
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin - self.center;
        let a: f64 = r.direction.squared_length();
        let half_b: f64 = oc * r.direction;
        let c: f64 = oc.squared_length() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root: f64 = discriminant.sqrt();

            let t1: f64 = (-half_b - root) / a;
            if t1 > t_min && t1 < t_max {
                rec.t = t1;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                //get_sphere_uv((rec.p - self.center) / self.radius, rec.u, rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }

            let t2: f64 = (-half_b + root) / a;
            if t2 > t_min && t2 < t_max {
                rec.t = t2;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                //get_sphere_uv((rec.p - self.center) / self.radius, rec.u, rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }

        return false;
    }
}

