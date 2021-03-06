use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::utils::*;
use crate::vec3::{self, Color, Point3, Vec3};
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::zero()
    }
}

// Lambertian

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction: Vec3 = rec.normal + vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);

        true
    }
}

// Metal

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Self {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = vec3::reflect(r_in.direction.unit(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + vec3::random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;

        scattered.direction * rec.normal > 0.0
    }
}

// Dielectric

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Self { ref_idx: ri }
    }

    fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let r: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0: f64 = r * r;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::ones();
        let etai_over_etat: f64 = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction: Vec3 = r_in.direction.unit();
        let cos_theta: f64 = fmin(1.0, -unit_direction * rec.normal);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected: Vec3 = vec3::reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }

        let reflect_prob: f64 = Self::schlick(cos_theta, etai_over_etat);
        if random_f64() < reflect_prob {
            let reflected: Vec3 = vec3::reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }

        let refracted: Vec3 = vec3::refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p, refracted);

        true
    }
}

// Diffuse light

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
