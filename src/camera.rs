use crate::vec3::{self, Vec3, Point3, Color};
use crate::ray::Ray;
use crate::utils::*;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(
        lookfrom: Point3, 
        lookat: Point3, 
        vup: Vec3, 
        vfov: f64, 
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64
    ) -> Self {
        
        let theta: f64 = degrees_to_radians(vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w: Vec3 = (lookfrom - lookat).unit();
        let u: Vec3 = vup.cross(w).unit();
        let v: Vec3 = w.cross(u);

        let origin: Point3 = lookfrom;
        let horizontal: Vec3 = u * viewport_width * focus_dist;
        let vertical: Vec3 = v * viewport_height * focus_dist;
        let lower_left_corner: Point3 = origin - horizontal/2.0 - vertical/2.0 - w * focus_dist;
        let lens_radius: f64 = aperture / 2.0;

        Self { origin, lower_left_corner, horizontal, vertical, u, v, w, lens_radius }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = vec3::random_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;

        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset)
    }
}