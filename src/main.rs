#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod utils;
mod hittable;
mod sphere;
mod camera;
mod material;
mod aabb;
mod texture;
mod aarect;
mod r#box;
mod bvh;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rusttype::Font;
use std::sync::{mpsc::channel, Arc};
use threadpool::ThreadPool;
pub use vec3::{Vec3, Point3, Color};
pub use ray::Ray;
pub use utils::*;
pub use hittable::{HitRecord, Hittable, HittableList};
pub use sphere::Sphere;
pub use camera::Camera;
pub use material::{Material, Lambertian, Metal, Dielectric, DiffuseLight};
pub use aabb::AABB;
pub use texture::{Texture, SolidColor, CheckerTexture};
pub use aarect::{XyRect, YzRect, XzRect};
pub use r#box::Box;
pub use bvh::BvhNode;

// x-axis: horizontal (left -> right)
// y-axis: vertical (top -> bottom)

const AUTHOR: &str = "Yuanxin Cao";



fn get_text() -> String {
    // GITHUB_SHA is the associated commit ID
    // only available on GitHub Action
    let github_sha = option_env!("GITHUB_SHA")
        .map(|x| "@".to_owned() + &x[0..6])
        .unwrap_or_default();
    format!("{}{}", AUTHOR, github_sha)
}

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn render_text(image: &mut RgbImage, msg: &str) {
    let font_file = if is_ci() {
        "EncodeSans-Regular.ttf"
    } else {
        "/System/Library/Fonts/Helvetica.ttc"
    };
    let font_path = std::env::current_dir().unwrap().join(font_file);
    let data = std::fs::read(&font_path).unwrap();
    let font: Font = Font::try_from_vec(data).unwrap_or_else(|| {
        panic!(format!(
            "error constructing a Font from data at {:?}",
            font_path
        ));
    });

    imageproc::drawing::draw_text_mut(
        image,
        Rgb([255, 255, 255]),
        10,
        10,
        rusttype::Scale::uniform(24.0),
        &font,
        msg,
    );
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    // Ground
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1000.0, 0.0), 1000.0, 
              Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9))))))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_f64();
            let center = Point3::new(a as f64 + 0.9 * random_f64(), -0.2, b as f64 + 0.9 * random_f64());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    let albedo: Color = Color::random().elemul(Color::random());
                    world.add(Arc::new(Sphere::new(center, 0.2, Arc::new(Lambertian::new(Arc::new(SolidColor::new(albedo)))))));
                }

                else if choose_mat < 0.95 {
                    let albedo: Color = Color::random_range(0.5, 1.0);
                    let fuzz: f64 = random_f64_range(0.0, 0.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, Arc::new(Metal::new(albedo, fuzz)))));
                }

                else {
                    world.add(Arc::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    world.add(Arc::new(Sphere::new(Point3::new(0.0, -1.0, 0.0), 1.0, Arc::new(Dielectric::new(1.5)))));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, -1.0, 0.0), 1.0, Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(0.4, 0.2, 0.1))))))));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, -1.0, 0.0), 1.0, Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)))));

    return world;

}


fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    // Ground
    let checker: Color = Color::random();
    objects.add(Arc::new(Sphere::new(Point3::new(0.0, 1000.0, 0.0), 1000.0, 
              Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(checker, Color::new(0.9, 0.9, 0.9))))))));
    
    // Sphere
    let albedo: Color = Color::random().elemul(Color::random());
    objects.add(Arc::new(Sphere::new(Point3::new(2.0, -1.0, 1.0), 1.0, Arc::new(Lambertian::new(Arc::new(SolidColor::new(albedo)))))));
    //objects.add(Arc::new(Sphere::new(Point3::new(2.0, -1.0, 1.0), 1.0, Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(4.0, 4.0, 4.0))))))));
    
    // Light
    objects.add(Arc::new(XyRect::new(3.0, 5.0, -3.0, -1.0, -2.0, Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(4.0, 4.0, 4.0))))))));

    return objects;
}


fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    //let ground = Arc::new(Lambertian::new(Color::(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Box::new(Point3::new(x0,y0,z0), Point3::new(x1,y1,z1), 
                      Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(0.48, 0.83, 0.53))))))));
        }
    }

    let mut objects = HittableList::new();
    objects.add(Arc::new(BvhNode::new(&mut boxes1)));

    // light
    objects.add(Arc::new(XzRect::new(123.0, 423.0, 147.0, 412.0, -554.0,
                         Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(7.0, 7.0, 7.0))))))));

    return objects;
}


fn ray_color(r: Ray, background: Color, world: &impl Hittable, depth: u32) -> Color {
    let mut rec = HitRecord::new(Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::zero())))));

    if depth <= 0 {
        return Color::zero();
    }

    if !world.hit(r, 0.001, INF, &mut rec) {
        return background;
    }

    let mut scattered = Ray::new(Point3::zero(), Vec3::zero());
    let mut attenuation =  Color::zero();
    let emitted: Color = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);

    if !rec.mat_ptr.scatter(r, &rec, &mut attenuation, &mut scattered) {
        return emitted;
    }

    return emitted + attenuation.elemul(ray_color(scattered, background, world, depth-1));

}

fn get_color(x: u32, y: u32, 
             image_width: u32, 
             image_height: u32, 
             samples_per_pixel: u32, 
             cam: &Camera, 
             my_world: &HittableList,
             background: Color,
             max_depth: u32) -> [u8; 3] {

    let mut pixel_color: Color = Color::zero();
    for s in 0..samples_per_pixel {
        let u: f64 = x as f64 / (image_width - 1) as f64;
        let v: f64 = y as f64/ (image_height - 1) as f64;
        let r = (*cam).get_ray(u, v);
        pixel_color += ray_color(r, background, my_world, max_depth);
    }  
    pixel_color = pixel_color / (samples_per_pixel as f64);

    [(clamp(pixel_color.x.sqrt(), 0.0, 0.999) * 256.0) as u8, 
     (clamp(pixel_color.y.sqrt(), 0.0, 0.999) * 256.0) as u8,
     (clamp(pixel_color.z.sqrt(), 0.0, 0.999) * 256.0) as u8]

}


fn main() {

    // get environment variable CI, which is true for GitHub Action
    let is_ci = is_ci();

    // jobs: split image into how many parts
    // workers: maximum allowed concurrent running threads
    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 2) } else { (16, 2) };

    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let width: u32 = 400;
    let height: u32 = ((width as f64) / aspect_ratio) as u32; 
    let samples_per_pixel: u32 = 40;
    let max_depth: u32 = 30;


    // Camera

    let lookfrom: Point3 = Point3::new(26.0, -3.0, 6.0);
    let lookat: Point3 = Point3::new(0.0, -2.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.0;
    let background = Color::new(0.5, 0.5, 0.5);

    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);


    // World

    let world = random_scene();


    // create a channel to send objects between threads

    let (tx, rx) = channel();
    let pool = ThreadPool::new(n_workers);
    let bar = ProgressBar::new(n_jobs as u64);


    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ = world.clone();
        pool.execute(move || {
            // here, we render some of the rows of image in one thread
            let row_begin = height as usize * i / n_jobs;
            let row_end = height as usize * (i + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(width, render_height as u32);
            for x in 0..width {
                // img_y is the row in partial rendered image
                // y is real position in final image
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y as u32;
                    let pixel = img.get_pixel_mut(x, img_y as u32);
                    let color = get_color(x, y, width, height, samples_per_pixel, &cam, &world_, background, max_depth);
                    *pixel = Rgb(color);
                }
            }
            // send row range and rendered image to main thread
            tx.send((row_begin..row_end, img)).expect("failed to send result");
        });
    }

    let mut result: RgbImage = ImageBuffer::new(width, height);

    for (rows, data) in rx.iter().take(n_jobs) {
        // idx is the corrsponding row in partial-rendered image
        for (idx, row) in rows.enumerate() {
            for col in 0..width {
                let row = row as u32;
                let idx = idx as u32;
                *result.get_pixel_mut(col, row) = *data.get_pixel(col, idx);
            }
        }
        bar.inc(1);
    }
    bar.finish();

    // render commit ID and author name on image
    let msg = get_text();
    println!("Extra Info: {}", msg);

    //render_text(&mut result, msg.as_str());

    result.save("output/random_test.png").unwrap();

}

