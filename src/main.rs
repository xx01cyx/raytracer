#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod utils;
mod hittable;
mod sphere;
mod camera;
mod material;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rusttype::Font;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;
pub use vec3::{Vec3, Point3, Color};
pub use ray::Ray;
pub use utils::*;
pub use hittable::{HitRecord, Hittable, HittableList};
pub use sphere::Sphere;
pub use camera::Camera;
pub use material::{Material, Lambertian, Metal, Dielectric};


// x-axis: horizontal (left -> right)
// y-axis: vertical (top -> bottom)

const AUTHOR: &str = "Yuanxin Cao";

struct World {
    pub height: u32,
}

impl World {
    pub fn color(&self, _: u32, y: u32) -> u8 {
        (y * 256 / self.height) as u8
    }
}

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

fn ray_color(r: Ray, world: &impl Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::new(Arc::new(Lambertian::new(Color::zero())));

    if depth <= 0 {
        return Color::zero();
    }

    if world.hit(r, 0.001, INF, &mut rec) {
        let mut scattered = Ray::new(Point3::zero(), Vec3::zero());
        let mut attenuation =  Color::zero();
        if rec.mat_ptr.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation.elemul(ray_color(scattered, world, depth-1));
        }
        return Color::zero();
        //let target: Point3 = rec.p + vec3::random_in_hemisphere(rec.normal);
        //return ray_color(Ray::new(rec.p, target-rec.p), world, depth-1) * 0.5;
    }

    let unit_direction: Vec3 = r.direction.unit();
    let t: f64 = (unit_direction.y + 1.0) * 0.5;
    return Color::ones() * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32; 
    let samples_per_pixel: i32 = 5;
    let max_depth: i32 = 20;


    // Camera

    let lookfrom: Point3 = Point3::new(3.0, -3.0, 2.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, -1.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = (lookfrom - lookat).length();
    let aperture: f64 = 2.0;

    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);    
    

    // World

    let mut world = HittableList::new();

    world.add(Arc::new(Sphere::new(Point3::new(0.0, 100.5, -1.0), 100.0, Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))))));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))))));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Arc::new(Dielectric::new(1.5)))));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, Arc::new(Dielectric::new(1.5)))));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0)))));


    // Render

    let mut result: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color: Color = Color::zero();
            for s in 0..samples_per_pixel {
                let u: f64 = i as f64 / (image_width - 1) as f64;
                let v: f64 = j as f64/ (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }  
            pixel_color = pixel_color / (samples_per_pixel as f64);
            *result.get_pixel_mut(i as u32, j as u32) = Rgb([(clamp(pixel_color.x.sqrt(), 0.0, 0.999) * 256.0) as u8, 
                                                             (clamp(pixel_color.y.sqrt(), 0.0, 0.999) * 256.0) as u8,
                                                             (clamp(pixel_color.z.sqrt(), 0.0, 0.999) * 256.0) as u8])
        }
    }

    result.save("output/camera.png").unwrap();

    
    /*
    // get environment variable CI, which is true for GitHub Action
    let is_ci = is_ci();

    // jobs: split image into how many parts
    // workers: maximum allowed concurrent running threads
    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 2) } else { (16, 2) };

    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );

    let height = 512;
    let width = 1024;

    // create a channel to send objects between threads
    let (tx, rx) = channel();
    let pool = ThreadPool::new(n_workers);

    let bar = ProgressBar::new(n_jobs as u64);

    // use Arc to pass one instance of World to multiple threads
    let world = Arc::new(World { height });

    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ptr = world.clone();
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
                    let color = world_ptr.color(x, y);
                    *pixel = Rgb([color, color, color]);
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

    //render_text(&mut result, msg.as_str());

    result.save("output/test.png").unwrap();
    */

}
