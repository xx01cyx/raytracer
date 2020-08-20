use crate::ray::Ray;
use crate::utils::*;
use crate::vec3::Point3;
use std::mem;

#[derive(Clone)]
pub struct AABB {
    pub _min: Point3,
    pub _max: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self { _min: a, _max: b }
    }

    pub fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let inv_d: f64 = 1.0 / r.direction.axis(a);
            let mut t0: f64 = (self._min.axis(a) - r.origin.axis(a)) * inv_d;
            let mut t1: f64 = (self._max.axis(a) - r.origin.axis(a)) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }

            let tmin = if t0 > tmin { t0 } else { tmin };
            let tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }

        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small: Point3 = Point3::new(
        fmin(box0._min.x, box1._min.x),
        fmin(box0._min.y, box1._min.y),
        fmin(box0._min.z, box1._min.z),
    );
    let big: Point3 = Point3::new(
        fmax(box0._max.x, box1._max.x),
        fmax(box0._max.y, box1._max.y),
        fmax(box0._max.z, box1._max.z),
    );

    AABB::new(small, big)
}
