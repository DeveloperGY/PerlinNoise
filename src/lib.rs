#![warn(missing_docs)]

//! A basic implementation of the 2D perlin noise algorithm for learning purposes
//! It is essentially a copy of the wikipedia page https://en.wikipedia.org/wiki/Perlin_noise

use na::Vector2;
use nalgebra as na;
use rand::{rngs::StdRng, Rng, SeedableRng};

/// The main interface for generating 2d perlin noise values
pub struct Perlin2D;

impl Perlin2D {
    /// Creates a new Perlin2D with a random seed
    pub fn new() -> Self {
        Self
    }

    /// A linear interpolation function.
    ///
    /// Interpolates between a and b, based on t.
    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        if t <= 0.0 {
            return a;
        }
        if t >= 1.0 {
            return b;
        }

        a + (b - a) * t
    }

    /// Generates a vector in a random direction based on the given coordinates
    fn random_gradient(x: i32, y: i32) -> na::Vector2<f32> {
        let seed = (x * 1093738) ^ (y * 185936289);
        let number: f32 = StdRng::seed_from_u64(seed as u64).gen_range(0.0..=3.1415926535);

        let x = number.cos();
        let y = number.sin();

        na::Vector2::new(x, y)
    }

    /// Finds the dot product between the gradient vector and the distance vector
    fn dot_grid_gradient(x: f32, y: f32, ix: i32, iy: i32) -> f32 {
        let gradient: na::Vector2<f32> = Self::random_gradient(ix, iy);

        let dx = x - ix as f32;
        let dy = y - iy as f32;
        let distance: Vector2<f32> = Vector2::new(dx, dy);

        distance.dot(&gradient)
    }

    /// Generates a perlin noise value for the given coordinates
    pub fn perlin(&self, x: f32, y: f32) -> f32 {
        let x0 = x.floor() as i32;
        let x1 = x0 + 1;
        let y0 = y.floor() as i32;
        let y1 = y0 + 1;

        let tx = x - x0 as f32;
        let ty = y - y0 as f32;

        let x0y0 = Self::dot_grid_gradient(x, y, x0, y0);
        let x1y0 = Self::dot_grid_gradient(x, y, x1, y0);

        let x0y1 = Self::dot_grid_gradient(x, y, x0, y1);
        let x1y1 = Self::dot_grid_gradient(x, y, x1, y1);

        println!("{}\n{}\n{}\n{}\n", x0y0, x1y0, x0y1, x1y1);

        let bottom_grad = Self::lerp(x0y0, x1y0, tx);
        let top_grad = Self::lerp(x0y1, x1y1, tx);

        println!("{}\n{}\n", bottom_grad, top_grad);

        let grad = Self::lerp(bottom_grad, top_grad, ty);

        grad
    }
}
