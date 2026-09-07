mod fractals; // Points to src/fractals.rs

use fractals::{EngineInternal, Bounds, CANVAS_WIDTH, CANVAS_HEIGHT};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FractalEngine;

#[wasm_bindgen]
impl FractalEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FractalEngine {
        FractalEngine
    }

    pub fn generate(&self, kind: u8, bounds: Bounds, max_iterations: i32) -> Result<js_sys::Float64Array, JsValue> {
        let internal = EngineInternal;
        let points = match kind {
            1 => internal.generate_mandelbrot(bounds, max_iterations),
            2 => internal.generate_julia(bounds, max_iterations),
            3 => internal.generate_leaf(),
            _ => return Err(JsValue::from_str("Invalid fractal kind")),
        };

        let mut flat_data = Vec::with_capacity(points.len() * 3);
        for pt in points {
            flat_data.push(pt.x);
            flat_data.push(pt.y);
            flat_data.push(pt.intensity as f64);
        }

        Ok(js_sys::Float64Array::from(&flat_data[..]))
    }
}