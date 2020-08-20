mod aabb;
mod aarect;
mod r#box;
mod bvh;
mod camera;
mod hittable;
mod material;
mod ray;
mod scene;
mod sphere;
mod texture;
mod utils;
#[allow(clippy::float_cmp)]
mod vec3;
pub use aabb::AABB;
pub use aarect::{XyRect, XzRect, YzRect};
pub use bvh::BvhNode;
pub use camera::Camera;
pub use hittable::{HitRecord, Hittable, HittableList, RotateY};
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
pub use material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
pub use r#box::Box;
pub use ray::Ray;
pub use scene::*;
pub use sphere::Sphere;
use std::sync::{mpsc::channel, Arc};
pub use texture::{CheckerTexture, SolidColor, Texture};
use threadpool::ThreadPool;
pub use utils::*;
pub use vec3::{Color, Point3, Vec3};

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

fn ray_color(r: Ray, background: Color, world: &impl Hittable, depth: u32) -> Color {
    let mut rec = HitRecord::new(Arc::new(Lambertian::new(Arc::new(SolidColor::new(
        Color::zero(),
    )))));

    if depth == 0 {
        return Color::zero();
    }

    if !world.hit(r, 0.001, INF, &mut rec) {
        return background;
    }

    let mut scattered = Ray::new(Point3::zero(), Vec3::zero());
    let mut attenuation = Color::zero();
    let emitted: Color = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);

    if !rec
        .mat_ptr
        .scatter(r, &rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }

    emitted + attenuation.elemul(ray_color(scattered, background, world, depth - 1))
}

fn get_color(
    x: u32,
    y: u32,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    cam: &Camera,
    my_world: &HittableList,
    background: Color,
    max_depth: u32,
) -> [u8; 3] {
    let mut pixel_color: Color = Color::zero();
    for _ in 0..samples_per_pixel {
        let u: f64 = x as f64 / (image_width - 1) as f64;
        let v: f64 = y as f64 / (image_height - 1) as f64;
        let r = (*cam).get_ray(u, v);
        pixel_color += ray_color(r, background, my_world, max_depth);
    }
    pixel_color = pixel_color / (samples_per_pixel as f64);

    [
        (clamp(pixel_color.x.sqrt(), 0.0, 0.999) * 256.0) as u8,
        (clamp(pixel_color.y.sqrt(), 0.0, 0.999) * 256.0) as u8,
        (clamp(pixel_color.z.sqrt(), 0.0, 0.999) * 256.0) as u8,
    ]
}

fn main() {
    // get environment variable CI, which is true for GitHub Action
    let is_ci = is_ci();

    // jobs: split image into how many parts
    // workers: maximum allowed concurrent running threads
    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 16) } else { (16, 16) };

    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );

    let mut filename: &str = "";

    let mut aspect_ratio = 16.0 / 9.0;
    let mut width: u32 = 800;
    let mut samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;

    let mut world = HittableList::new();

    // Camera

    let mut lookfrom: Point3 = Point3::zero();
    let mut lookat: Point3 = Point3::zero();
    let mut aperture: f64 = 0.0;
    let mut background = Color::zero();
    let mut vfov: f64 = 40.0;

    let choice: i32 = 4;
    match choice {
        1 => {
            world = random_scene();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, -2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
            filename = "random_scene.png";
        }

        2 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color::zero();
            lookfrom = Point3::new(26.0, -3.0, 6.0);
            lookat = Point3::new(0.0, -2.0, 0.0);
            vfov = 20.0;
            filename = "simple_light.png";
        }

        3 => {
            world = final_scene();
            aspect_ratio = 1.0;
            width = 800;
            samples_per_pixel = 200;
            background = Color::zero();
            lookfrom = Point3::new(478.0, -278.0, -600.0);
            lookat = Point3::new(278.0, -278.0, 0.0);
            vfov = 40.0;
            filename = "final_scene.png";
        }

        4 => {
            world = maiden_room();
            samples_per_pixel = 400;
            background = Color::zero();
            lookfrom = Point3::new(26.0, -26.0, 6.0);
            lookat = Point3::new(0.0, -2.3, 0.0);
            vfov = 15.0;
            filename = "maiden_room.png";
        }

        _ => {}
    };

    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let height: u32 = ((width as f64) / aspect_ratio) as u32;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
                    let color = get_color(
                        x,
                        y,
                        width,
                        height,
                        samples_per_pixel,
                        &cam,
                        &world_,
                        background,
                        max_depth,
                    );
                    *pixel = Rgb(color);
                }
            }
            // send row range and rendered image to main thread
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
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

    let savepath = format!("output/{}", filename);
    result.save(savepath).unwrap();
}
