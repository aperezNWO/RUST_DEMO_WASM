mod fractals;

use fractals::{Engine, FractalKind, Bounds};

fn main() {
    let engine = Engine::new();
    
    // Example bounds for Mandelbrot
    let bounds = Bounds {
        x_min: -2.0,
        x_max: 1.0,
        y_min: -1.5,
        y_max: 1.5,
    };

    let points = engine.get_fractal(FractalKind::Mandelbrot, bounds, 200);
    println!("Successfully generated {} points.", points.len());
}