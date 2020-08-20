use crate::aarect::{XyRect, XzRect};
use crate::bvh::BvhNode;
use crate::hittable::{HittableList, RotateY};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::r#box::Box;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, SolidColor};
use crate::utils::*;
use crate::vec3::{Color, Point3};
use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    // Ground
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        )))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                -0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo: Color = Color::random().elemul(Color::random());
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::new(Arc::new(SolidColor::new(albedo)))),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo: Color = Color::random_range(0.5, 1.0);
                    let fuzz: f64 = random_f64_range(0.0, 0.5);
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, -1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
            0.4, 0.2, 0.1,
        ))))),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, -1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    // Ground
    let checker: Color = Color::random();
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(
            checker,
            Color::new(0.9, 0.9, 0.9),
        )))),
    )));

    // Sphere
    objects.add(Arc::new(Box::new(
        Point3::new(1.0, -2.0, 0.0),
        Point3::new(3.0, 0.0, 2.0),
        Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
            0.48, 0.83, 0.53,
        ))))),
    )));

    // Light
    objects.add(Arc::new(XyRect::new(
        3.0,
        5.0,
        -3.0,
        -1.0,
        -2.0,
        Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
            4.0, 4.0, 4.0,
        ))))),
    )));
    objects.add(Arc::new(XzRect::new(
        -1.0,
        5.0,
        -2.0,
        4.0,
        -4.0,
        Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
            4.0, 4.0, 4.0,
        ))))),
    )));

    objects
}

pub fn final_scene() -> HittableList {
    let mut objects = HittableList::new();
    let mut boxes = HittableList::new();

    let boxes_per_side = 10;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -500.0 + i as f64 * w;
            let z0 = -500.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes.add(Arc::new(Box::new(
                Point3::new(x0, -y1, z0),
                Point3::new(x1, -y0, z1),
                Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
                    0.48, 0.83, 0.53,
                ))))),
            )));
        }
    }

    objects.add(Arc::new(BvhNode::new(&mut boxes)));

    objects.add(Arc::new(XzRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        -554.0,
        Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
            7.0, 7.0, 7.0,
        ))))),
    )));

    let albedo: Color = Color::random().elemul(Color::random());
    objects.add(Arc::new(Sphere::new(
        Point3::new(260.0, -150.0, 45.0),
        50.0,
        Arc::new(Lambertian::new(Arc::new(SolidColor::new(albedo)))),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 10.0)),
    )));

    objects
}

pub fn maiden_room() -> HittableList {
    let mut room = HittableList::new();

    // Ground
    
    let misty_rose = Color::new(255.0, 228.0, 225.0) / 255.0;
    let light_gray = Color::new(211.0, 211.0, 211.0) / 255.0;
    let ground_material = Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(misty_rose, light_gray))));
    room.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1500.0, 0.0),
        1500.0,
        ground_material,
    )));

    // Spheres

    for a in -12..12 {
        for b in -12..12 {
            let choose_mat: f64 = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                -0.2,
                b as f64 + 0.9 * random_f64(),
            );
            let radius = random_f64_range(0.15, 0.35);

            if (center.x >= -1.5 && center.x <= 1.5) && (center.z >= -1.5 && center.z <= 1.5) {
                continue;
            }

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.05 {
                    room.add(Arc::new(Sphere::new(
                        center,
                        radius,
                        Arc::new(Dielectric::new(0.5)),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo: Color = Color::ones() - Color::random().elemul(Color::random());
                    room.add(Arc::new(Sphere::new(
                        center,
                        radius,
                        Arc::new(Lambertian::new(Arc::new(SolidColor::new(albedo)))),
                    )));
                } else {
                    let albedo: Color = Color::random_range(0.5, 1.0);
                    let fuzz: f64 = random_f64_range(0.0, 0.5);
                    room.add(Arc::new(Sphere::new(
                        center,
                        radius,
                        Arc::new(Metal::new(albedo, fuzz)),
                    )));
                }
            }
        }
    }

    // Light

    room.add(Arc::new(Sphere::new(
        Point3::new(-4.0, -13.0, 8.0),
        5.0,
        Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
            7.0, 7.0, 7.0,
        ))))),
    )));
    room.add(Arc::new(Sphere::new(
        Point3::new(4.0, -18.0, -8.0),
        5.0,
        Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
            7.0, 7.0, 7.0,
        ))))),
    )));

    // Tower

    let msb = Color::new(123.0, 104.0, 238.0) / 255.0; // MediumSlateBlue
    let lc = Color::new(240.0, 128.0, 128.0) / 255.0; // LightCoral
    let lavender = Color::new(230.0, 230.0, 250.0) / 255.0; // Lavender

    let box1 = Box::new(
        Point3::new(-1.0, -2.0, -1.0),
        Point3::new(1.0, 0.0, 1.0),
        Arc::new(DiffuseLight::new(Arc::new(CheckerTexture::new(
            gradient_ramp(msb, lc, 6, 0),
            lavender,
        )))),
    );
    let box2 = Box::new(
        Point3::new(-0.707, -3.414, -0.707),
        Point3::new(0.707, -2.0, 0.707),
        Arc::new(DiffuseLight::new(Arc::new(CheckerTexture::new(
            gradient_ramp(msb, lc, 6, 1),
            lavender,
        )))),
    );
    let box3 = Box::new(
        Point3::new(-0.5, -4.414, -0.5),
        Point3::new(0.5, -3.414, 0.5),
        Arc::new(DiffuseLight::new(Arc::new(CheckerTexture::new(
            gradient_ramp(msb, lc, 6, 2),
            lavender,
        )))),
    );
    let sphere1 = Sphere::new(
        Point3::new(0.0, -4.714, 0.0),
        0.5,
        Arc::new(DiffuseLight::new(Arc::new(CheckerTexture::new(
            gradient_ramp(msb, lc, 6, 3),
            lavender,
        )))),
    );
    let box4 = Box::new(
        Point3::new(-0.1, -6.0, -0.1),
        Point3::new(0.1, -4.414, 0.1),
        Arc::new(DiffuseLight::new(Arc::new(CheckerTexture::new(
            gradient_ramp(msb, lc, 6, 4),
            lavender,
        )))),
    );
    let box5 = Box::new(
        Point3::new(-0.05, -6.8, -0.05),
        Point3::new(0.05, -4.414, 0.05),
        Arc::new(DiffuseLight::new(Arc::new(CheckerTexture::new(
            gradient_ramp(msb, lc, 6, 5),
            lavender,
        )))),
    );

    room.add(Arc::new(RotateY::new(Arc::new(box1), 90.0)));
    room.add(Arc::new(RotateY::new(Arc::new(box2), 45.0)));
    room.add(Arc::new(RotateY::new(Arc::new(box3), 90.0)));
    room.add(Arc::new(RotateY::new(Arc::new(box4), 45.0)));
    room.add(Arc::new(RotateY::new(Arc::new(box5), 45.0)));
    room.add(Arc::new(sphere1));

    room
}

fn gradient_ramp(c1: Color, c2: Color, num: u32, step: u32) -> Color {
    let x_tolerance: f64 = (c2.x - c1.x) / (num - 1) as f64;
    let y_tolerance: f64 = (c2.y - c1.y) / (num - 1) as f64;
    let z_tolerance: f64 = (c2.z - c1.z) / (num - 1) as f64;

    Color::new(
        c1.x + x_tolerance * step as f64,
        c1.y + y_tolerance * step as f64,
        c1.z + z_tolerance * step as f64,
    )
}
