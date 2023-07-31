use noise::{
    generators, transforms,
    utils::{NoiseBuffer, Visualizer},
};

fn main() {
    example_simplex_noise1d();
    example_simplex_noise2d();
    example_simplex_noise3d();
    example_simplex_noise4d();
}

fn example_simplex_noise1d() {
    let shape = &[100];
    let generator = transforms::inputscale::transform1d(generators::simplex::noise1d, 0.013);
    let noisebuf = NoiseBuffer::new1d(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("simplex_1d.png");
}

fn example_simplex_noise2d() {
    let shape = &[1000, 1000];
    let generator = transforms::inputscale::transform2d(generators::simplex::noise2d, 0.013);
    let noisebuf = NoiseBuffer::new2d(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("simplex_2d.png");
}

fn example_simplex_noise3d() {
    let shape = &[200, 200, 200];
    let generator = transforms::inputscale::transform3d(generators::simplex::noise3d, 0.013);
    let noisebuf = NoiseBuffer::new3d(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("simplex_3d.png");
}

fn example_simplex_noise4d() {
    let shape = &[60, 60, 60, 60];
    let generator = transforms::inputscale::transform4d(generators::simplex::noise4d, 0.033);
    let noisebuf = NoiseBuffer::new4d(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("simplex_4d.gif");
}