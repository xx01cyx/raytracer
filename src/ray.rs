use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let orig = Vec3::new(1.0, 1.0, 1.0);
        let dir = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(Ray::new(orig, dir), Ray::new(orig, dir));
    }

    #[test]
    fn test_at() {
        let orig = Vec3::new(3.0, 4.0, 5.0);
        let dir = Vec3::new(1.0, 1.0, 1.0);
        let r = Ray::new(orig, dir);
        assert_eq!(r.at(-2.0), Vec3::new(1.0, 2.0, 3.0));
    }
}