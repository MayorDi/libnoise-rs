use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Fbm<G> {
    generator: G,
    octaves: u32,
    frequency: f64,
    lacunarity: f64,
    persistence: f64,
    normalization_factor: f64,
}

impl<G: Generator<1>> Generator1D for Fbm<G> {}
impl<G: Generator<2>> Generator2D for Fbm<G> {}
impl<G: Generator<3>> Generator3D for Fbm<G> {}
impl<G: Generator<4>> Generator4D for Fbm<G> {}

impl<G> Fbm<G> {
    #[inline]
    pub fn new(
        generator: G,
        octaves: u32,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
    ) -> Self {
        let normalization_factor = compute_normalization_factor(octaves, persistence);
        Self {
            generator,
            octaves,
            frequency,
            lacunarity,
            persistence,
            normalization_factor,
        }
    }
}

macro_rules! impl_generator {
    ($dim:literal) => {
        impl<G: Generator<$dim>> Generator<$dim> for Fbm<G> {
            fn sample(&self, point: [f64; $dim]) -> f64 {
                let mut noise = 0.0;
                let mut amp = 1.0;
                let mut freq = self.frequency;
                for _ in 0..self.octaves {
                    noise += amp * self.generator.sample(point.map(|x| x * freq));
                    freq *= self.lacunarity;
                    amp *= self.persistence;
                }
                noise * self.normalization_factor
            }
        }
    };
}

impl_generator!(1);
impl_generator!(2);
impl_generator!(3);
impl_generator!(4);

#[inline]
fn compute_normalization_factor(octaves: u32, persistence: f64) -> f64 {
    1.0 / (0..octaves).fold(0.0, |acc, octave| acc + persistence.powi(octave as i32))
}
