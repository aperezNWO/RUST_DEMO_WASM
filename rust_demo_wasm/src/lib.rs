use wasm_bindgen::prelude::*;
use rand::Rng;

const CANVAS_WIDTH: usize = 800;
const CANVAS_HEIGHT: usize = 600;

#[wasm_bindgen]
pub struct FractalEngine {
    buffer: Vec<f64>,
}

#[wasm_bindgen]
impl FractalEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FractalEngine {
        FractalEngine {
            buffer: Vec::with_capacity(CANVAS_WIDTH * CANVAS_HEIGHT * 3),
        }
    }

    /// Returns a pointer to the start of the buffer in WebAssembly memory
    pub fn buffer_ptr(&self) -> *const f64 {
        self.buffer.as_ptr()
    }

    /// Returns the element count of the buffer
    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn generate(
        &mut self,
        kind: u8,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        max_iterations: i32,
    ) -> Result<(), JsValue> {
        self.buffer.clear();

        match kind {
            1 => Self::generate_mandelbrot(&mut self.buffer, x_min, x_max, y_min, y_max, max_iterations),
            2 => Self::generate_julia(&mut self.buffer, x_min, x_max, y_min, y_max, max_iterations),
            3 => Self::generate_leaf(&mut self.buffer),
            _ => return Err(JsValue::from_str("Invalid fractal kind")),
        };

        Ok(())
    }
}

impl FractalEngine {
    #[inline]
    fn encode_intensity(iter: i32, max_iterations: i32) -> f64 {
        if iter == max_iterations {
            return 0.0;
        }
        ((iter * 255) / max_iterations) as f64
    }

    fn generate_mandelbrot(
        buffer: &mut Vec<f64>,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        max_iterations: i32,
    ) {
        let x_range = x_max - x_min;
        let y_range = y_max - y_min;

        if x_range <= 0.0 || y_range <= 0.0 {
            return;
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

                buffer.push(screen_x as f64);
                buffer.push(screen_y as f64);
                buffer.push(Self::encode_intensity(iter, max_iterations));
            }
        }
    }

    fn generate_julia(
        buffer: &mut Vec<f64>,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        max_iterations: i32,
    ) {
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

                buffer.push(screen_x as f64);
                buffer.push(screen_y as f64);
                buffer.push(Self::encode_intensity(iter, max_iterations));
            }
        }
    }

    fn generate_leaf(buffer: &mut Vec<f64>) {
        let mut pixel_grid = [[0u8; CANVAS_HEIGHT]; CANVAS_WIDTH];
        let mut x = 0.0;
        let mut y = 0.0;
        let mut rng = rand::thread_rng();
        let total_points = 150_000;

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

        for px in 0..CANVAS_WIDTH {
            for py in 0..CANVAS_HEIGHT {
                if pixel_grid[px][py] > 0 {
                    buffer.push(px as f64);
                    buffer.push(py as f64);
                    buffer.push(pixel_grid[px][py] as f64);
                }
            }
        }
    }
}