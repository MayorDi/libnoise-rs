use image::{
    codecs::gif::{GifEncoder, Repeat},
    ColorType,
};
use itertools::Itertools;
use std::fs::OpenOptions;

#[test]
fn test_simplex_noise1d() {
    let noise = noise::generators::simplex::noise1d;
    test_noise1d(noise, "simplex_1d.png");
}

#[test]
fn test_simplex_noise2d() {
    let noise = noise::generators::simplex::noise2d;
    test_noise2d(noise, "simplex_2d.png");
}

#[test]
fn test_simplex_noise3d() {
    let noise = noise::generators::simplex::noise3d;
    test_noise3d(noise, "simplex_3d.gif");
}

#[test]
fn test_simplex_noise4d() {
    let noise = noise::generators::simplex::noise4d;
    test_noise4d(noise, "simplex_4d.gif");
}

#[test]
fn test_fractal_simplex_noise1d() {
    let noise = noise::transforms::fractal::transform1d(
        noise::generators::simplex::noise1d,
        4,
        1.0,
        1.0,
        2.0,
        0.5,
    );
    test_noise1d(noise, "fractal_simplex_1d.png");
}

#[test]
fn test_fractal_simplex_noise2d() {
    let noise = noise::transforms::fractal::transform2d(
        noise::generators::simplex::noise2d,
        4,
        1.0,
        1.0,
        2.0,
        0.5,
    );
    test_noise2d(noise, "fractal_simplex_2d.png");
}

#[test]
fn test_fractal_simplex_noise3d() {
    let noise = noise::transforms::fractal::transform3d(
        noise::generators::simplex::noise3d,
        4,
        1.0,
        1.0,
        2.0,
        0.5,
    );
    test_noise3d(noise, "fractal_simplex_3d.gif");
}

#[test]
fn test_fractal_simplex_noise4d() {
    let noise = noise::transforms::fractal::transform4d(
        noise::generators::simplex::noise4d,
        4,
        1.0,
        1.0,
        2.0,
        0.5,
    );
    test_noise4d(noise, "fractal_simplex_4d.gif");
}

fn test_noise1d<F>(generator: F, path: &str)
where
    F: Fn(u64, f64) -> f64,
{
    let shape = &[100];
    let generator = noise::transforms::inputscale::transform1d(generator, 0.013);
    let noisebuf = noise::utils::NoiseBuffer::new1d(shape, generator, 42);
    noise::utils::Visualizer::from(noisebuf).write_to_file(path);
}

fn test_noise2d<F>(generator: F, path: &str)
where
    F: Fn(u64, f64, f64) -> f64,
{
    let shape = &[3000, 3000];
    let generator = noise::transforms::inputscale::transform2d(generator, 0.013);
    let noisebuf = noise::utils::NoiseBuffer::new2d(shape, generator, 42);
    noise::utils::Visualizer::from(noisebuf).write_to_file(path);
}

fn test_noise3d<F>(generator: F, path: &str)
where
    F: Fn(u64, f64, f64, f64) -> f64,
{
    let shape = &[300, 300, 50];
    let generator = noise::transforms::inputscale::transform3d(generator, 0.033);
    let noisebuf = noise::utils::NoiseBuffer::new3d(shape, generator, 42);
    noise::utils::Visualizer::from(noisebuf).write_to_file(path);
}

fn test_noise4d<F>(generator: F, path: &str)
where
    F: Fn(u64, f64, f64, f64, f64) -> f64,
{
    let size = 50;
    let generator = noise::transforms::inputscale::transform4d(generator, 0.033);
    let noisebuf_yzw = noise::utils::NoiseBuffer::new4d(&[1, size, size, size], &generator, 42);
    let noisebuf_xzw = noise::utils::NoiseBuffer::new4d(&[size, 1, size, size], &generator, 42);
    let noisebuf_xyw = noise::utils::NoiseBuffer::new4d(&[size, size, 1, size], &generator, 42);
    let noisebuf_xyz = noise::utils::NoiseBuffer::new4d(&[size, size, size, 1], &generator, 42);
    let buffers = [noisebuf_yzw, noisebuf_xzw, noisebuf_xyw, noisebuf_xyz];
    let file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut encoder = GifEncoder::new(file_out);
    encoder.set_repeat(Repeat::Infinite).unwrap();
    let frame_w = 4 * size as u32;
    let frame_h = size as u32;
    for t in 0..size {
        let channel = [frame_h, frame_w]
            .iter()
            .map(|&dim_size| 0..dim_size)
            .multi_cartesian_product()
            .map(|p| {
                let p0 = p[0] as usize;
                let p1 = p[1] as usize;
                match p1 / size {
                    0 => buffers[0][&[0, p1 % size, p0, t]],
                    1 => buffers[1][&[p1 % size, 0, p0, t]],
                    2 => buffers[2][&[p1 % size, p0, 0, t]],
                    3 => buffers[3][&[p1 % size, p0, t, 0]],
                    _ => unreachable!(),
                }
            })
            .map(|val| (val * 172.5 + 172.5) as u8)
            .flat_map(|val| std::iter::repeat(val).take(3))
            .collect::<Vec<u8>>();
        encoder
            .encode(&channel, frame_w, frame_h, ColorType::Rgb8)
            .unwrap();
    }
}
