use rand::Rng;
use serde::{Deserialize, Serialize};

pub const CANVAS_WIDTH: usize = 800;
pub const CANVAS_HEIGHT: usize = 600;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FractalKind {
    Mandelbrot = 1,
    Julia = 2,
    Leaf = 3,
}

impl FractalKind {
    #[allow(dead_code)]
    pub fn from_i32(val: i32) -> Result<Self, String> {
        match val {
            1 => Ok(FractalKind::Mandelbrot),
            2 => Ok(FractalKind::Julia),
            3 => Ok(FractalKind::Leaf),
            _ => Err(format!("tipo de fractal inválido: {}", val)),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FractalPoint {
    pub x: f64,
    pub y: f64,
    pub intensity: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bounds {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Engine
    }

    pub fn get_fractal(&self, kind: FractalKind, bounds: Bounds, max_iterations: i32) -> Vec<FractalPoint> {
        match kind {
            FractalKind::Mandelbrot => self.generate_mandelbrot(bounds, max_iterations),
            FractalKind::Julia => self.generate_julia(bounds, max_iterations),
            FractalKind::Leaf => self.generate_leaf(),
        }
    }

    fn encode_intensity(iter: i32, max_iterations: i32) -> i32 {
        if iter == max_iterations {
            return 0;
        }
        (iter * 255) / max_iterations
    }

    pub fn generate_mandelbrot(&self, bounds: Bounds, max_iterations: i32) -> Vec<FractalPoint> {
        let mut points = Vec::with_capacity(CANVAS_WIDTH * CANVAS_HEIGHT);
        let x_range = bounds.x_max - bounds.x_min;
        let y_range = bounds.y_max - bounds.y_min;

        if x_range <= 0.0 || y_range <= 0.0 {
            return vec![];
        }

        for screen_y in 0..CANVAS_HEIGHT {
            for screen_x in 0..CANVAS_WIDTH {
                let c_re = bounds.x_min + (screen_x as f64 * x_range / CANVAS_WIDTH as f64);
                let c_im = bounds.y_min + (screen_y as f64 * y_range / CANVAS_HEIGHT as f64);

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

    pub fn generate_julia(&self, bounds: Bounds, max_iterations: i32) -> Vec<FractalPoint> {
        let mut points = Vec::with_capacity(CANVAS_WIDTH * CANVAS_HEIGHT);
        let x_range = bounds.x_max - bounds.x_min;
        let y_range = bounds.y_max - bounds.y_min;

        let c_re = -0.400;
        let c_im = 0.600;

        for screen_y in 0..CANVAS_HEIGHT {
            for screen_x in 0..CANVAS_WIDTH {
                let mut z_re = bounds.x_min + (screen_x as f64 * x_range / CANVAS_WIDTH as f64);
                let mut z_im = bounds.y_min + (screen_y as f64 * y_range / CANVAS_HEIGHT as f64);
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

    pub fn generate_leaf(&self) -> Vec<FractalPoint> {
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