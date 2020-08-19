use crate::vec3::{Vec3, Point3, Color};
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::aabb::{self, AABB};
use crate::utils::*;
use std::sync::Arc;
use std::cmp::Ordering;

pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bvh_box: AABB
}

impl BvhNode {
    fn new_(objects: &mut [Arc<dyn Hittable>], start: usize, end: usize) -> Self {
        for i in 0..objects.len() {
            let mut box_test = AABB::new(Point3::zero(), Point3::zero());
            println!("{}", objects[i].bounding_box(&mut box_test));
        }
        
        println!("start: {}", start);
        println!("end: {}", end);
        println!("objects.len(): {}", objects.len());
        let axis: i32 = random_i32_range(0, 2);
        println!("axis: {}", axis);
        let object_span: usize = end - start;
        println!("object_span: {}", object_span);
        let mut left: Arc<dyn Hittable> = Arc::new(HittableList::new());
        let mut right: Arc<dyn Hittable> = Arc::new(HittableList::new());
        
        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        }

        else if object_span == 2 {
            if box_compare(&objects[start], &objects[start+1], axis) {
                println!("left < right");
                left = objects[start].clone();
                right = objects[start+1].clone();
                let mut box_test = AABB::new(Point3::zero(), Point3::zero());
                println!("left.bouding_box: {}", left.bounding_box(&mut box_test));
                println!("right.bouding_box: {}", right.bounding_box(&mut box_test));
            }
            else {
                println!("left > right");
                left = objects[start+1].clone();
                right = objects[start].clone();
                let mut box_test = AABB::new(Point3::zero(), Point3::zero());
                println!("left.bouding_box: {}", left.bounding_box(&mut box_test));
                println!("right.bouding_box: {}", right.bounding_box(&mut box_test));
            }
        }

        else {
            objects[start..end].sort_by(|a, b| comparator(&a, &b, axis));
            println!("sorted");
            let mid: usize = start + object_span / 2;
            left = Arc::new(BvhNode::new_(&mut objects[..], start, mid));
            right = Arc::new(BvhNode::new_(&mut objects[..], mid, end));
            println!("OK!");
        }

        let mut box_left = AABB::new(Point3::zero(), Point3::zero());
        let mut box_right = AABB::new(Point3::zero(), Point3::zero());
        
        if !left.bounding_box(&mut box_left) || !right.bounding_box(&mut box_right) { 
            panic!("No bounding box in bvh_node constructor!");
        }
        let bvh_box: AABB = aabb::surrounding_box(&box_left, &box_right);
        println!("bvh_box._min: ({}, {}, {})", bvh_box._min.x, bvh_box._min.y, bvh_box._min.z);
        println!("bvh_box._max: ({}, {}, {})", bvh_box._max.x, bvh_box._max.y, bvh_box._max.z);


        Self { left, right, bvh_box }

    }   
    
    pub fn new(list: &mut HittableList) -> Self {
        let len: usize = list.objects.len();
        BvhNode::new_(&mut list.objects[..], 0, len)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bvh_box.hit(r, t_min, t_max) {
            return false;
        }

        let old_rec = rec.clone();
        let hit_left: bool = self.left.hit(r, t_min, t_max, rec);
        let hit_right: bool = self.right.hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);

        return hit_left || hit_right;
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        *output_box = self.bvh_box.clone();
        return true;
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, ax: i32) -> bool {
    let mut box_a: AABB = AABB::new(Point3::zero(), Point3::zero());
    let mut box_b: AABB = AABB::new(Point3::zero(), Point3::zero());

    if !a.bounding_box(&mut box_a) || !b.bounding_box(&mut box_b) { 
        panic!("No bounding box in bvh_node constructor!")
    }

    return box_a._min.axis(ax) < box_b._min.axis(ax);
      
}

fn comparator(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, ax: i32) -> Ordering {
    match box_compare(a, b, ax) {
        true => Ordering::Less,
        false => Ordering::Greater
    }
}

