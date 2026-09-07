use wasm_bindgen::prelude::*;
use rand::Rng;

const CANVAS_WIDTH: usize = 800;
const CANVAS_HEIGHT: usize = 600;

#[wasm_bindgen]
pub struct FractalEngine;

#[wasm_bindgen]
impl FractalEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FractalEngine {
        FractalEngine
    }

    /// Generates the fractal using primitive boundary inputs to avoid Wasm ABI mapping errors,
    /// returning a flat Float64Array [x1, y1, intensity1, x2, y2, intensity2, ...]
    pub fn generate(
        &self,
        kind: u8,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        max_iterations: i32,
    ) -> Result<js_sys::Float64Array, JsValue> {
        let points = match kind {
            1 => Self::generate_mandelbrot(x_min, x_max, y_min, y_max, max_iterations),
            2 => Self::generate_julia(x_min, x_max, y_min, y_max, max_iterations),
            3 => Self::generate_leaf(),
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

struct FractalPoint {
    x: f64,
    y: f64,
    intensity: i32,
}

impl FractalEngine {
    fn encode_intensity(iter: i32, max_iterations: i32) -> i32 {
        if iter == max_iterations {
            return 0;
        }
        (iter * 255) / max_iterations
    }

    fn generate_mandelbrot(x_min: f64, x_max: f64, y_min: f64, y_max: f64, max_iterations: i32) -> Vec<FractalPoint> {
        let mut points = Vec::with_capacity(CANVAS_WIDTH * CANVAS_HEIGHT);
        let x_range = x_max - x_min;
        let y_range = y_max - y_min;

        if x_range <= 0.0 || y_range <= 0.0 {
            return vec![];
        }

        for screen_y in 0..CANVAS_HEIGHT {
            for screen_x in 0..CANVAS_WIDTH {
                let c_re = x_min + (screen_x as f64 * x_range / CANVAS_WIDTH as f64);
                let c_im = y_min + (screen_y as f64 * y_range / CANVAS_HEIGHT as f64);

                let mut z_re = 0.0;
                let mut z_im = 0.0;
                let mut iter = 0;

                while z_re * z_re + z_im * z_im <= 4.0 && iter < max_iterations {
                    let next_re = z_re * z_re - z_im * z_im + c_re;
                    let next_im = 2.0 * z_re * z_im + c_im;
                    z_re = next_re;
                    z_im = next_im;
                    iter += 1;
                }

                points.push(FractalPoint {
                    x: screen_x as f64,
                    y: screen_y as f64,
                    intensity: Self::encode_intensity(iter, max_iterations),
                });
            }
        }
        points
    }

    fn generate_julia(x_min: f64, x_max: f64, y_min: f64, y_max: f64, max_iterations: i32) -> Vec<FractalPoint> {
        let mut points = Vec::with_capacity(CANVAS_WIDTH * CANVAS_HEIGHT);
        let x_range = x_max - x_min;
        let y_range = y_max - y_min;
        let c_re = -0.400;
        let c_im = 0.600;

        for screen_y in 0..CANVAS_HEIGHT {
            for screen_x in 0..CANVAS_WIDTH {
                let mut z_re = x_min + (screen_x as f64 * x_range / CANVAS_WIDTH as f64);
                let mut z_im = y_min + (screen_y as f64 * y_range / CANVAS_HEIGHT as f64);
                let mut iter = 0;

                while z_re * z_re + z_im * z_im <= 4.0 && iter < max_iterations {
                    let next_re = z_re * z_re - z_im * z_im + c_re;
                    let next_im = 2.0 * z_re * z_im + c_im;
                    z_re = next_re;
                    z_im = next_im;
                    iter += 1;
                }

                points.push(FractalPoint {
                    x: screen_x as f64,
                    y: screen_y as f64,
                    intensity: Self::encode_intensity(iter, max_iterations),
                });
            }
        }
        points
    }

    fn generate_leaf() -> Vec<FractalPoint> {
        let mut pixel_grid = [[0; CANVAS_HEIGHT]; CANVAS_WIDTH];
        let mut x = 0.0;
        let mut y = 0.0;
        let mut rng = rand::thread_rng();
        let total_points = 150000;

        for _ in 0..total_points {
            let next_x;
            let next_y;
            let r: f64 = rng.gen_range(0.0..100.0);

            if r < 1.0 {
                next_x = 0.0;
                next_y = 0.16 * y;
            } else if r < 86.0 {
                next_x = 0.85 * x + 0.04 * y;
                next_y = -0.04 * x + 0.85 * y + 1.6;
            } else if r < 93.0 {
                next_x = 0.20 * x - 0.26 * y;
                next_y = 0.23 * x + 0.22 * y + 1.6;
            } else {
                next_x = -0.15 * x + 0.28 * y;
                next_y = 0.26 * x + 0.24 * y + 0.44;
            }

            x = next_x;
            y = next_y;

            let screen_x = ((x + 2.182) * (CANVAS_WIDTH as f64 - 1.0) / (2.655 + 2.182)).round() as isize;
            let screen_y = ((9.96 - y) * (CANVAS_HEIGHT as f64 - 1.0) / 9.96).round() as isize;

            if screen_x >= 0 && screen_x < CANVAS_WIDTH as isize && screen_y >= 0 && screen_y < CANVAS_HEIGHT as isize {
                pixel_grid[screen_x as usize][screen_y as usize] = 200;
            }
        }

        let mut points = Vec::new();
        for px in 0..CANVAS_WIDTH {
            for py in 0..CANVAS_HEIGHT {
                if pixel_grid[px][py] > 0 {
                    points.push(FractalPoint {
                        x: px as f64,
                        y: py as f64,
                        intensity: pixel_grid[px][py],
                    });
                }
            }
        }
        points
    }
}