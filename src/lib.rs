mod utils;
use wasm_bindgen::prelude::*;
use rand::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const N: usize = 2500;

#[wasm_bindgen]
pub struct Universe {
    t: f64,
    x: [f32; N],
    y: [f32; N],
    vx: [f32; N],
    vy: [f32; N],
    ax: [f32; N],
    ay: [f32; N],
    m: [f32; N],
}

pub const DT: f32 = 0.01f32;
pub const WIDTH: u64 = 1024;
pub const HEIGHT: u64 = 1024;

#[wasm_bindgen]
impl Universe {

    pub fn new() -> Self {
        let mut x: [f32; self::N] = [0.0; self::N];
        let mut y: [f32; self::N] = [0.0; self::N];
        let mut vx: [f32; self::N] = [0.0; self::N];
        let mut vy: [f32; self::N] = [0.0; self::N];
        let m: [f32; self::N] = [3.0; self::N];
        let mut random = XorShiftRng::seed_from_u64(0);
        for i in 0..self::N {
            x[i] = random.gen_range(0.2f32*self::WIDTH as f32, 0.8f32 * self::WIDTH as f32);
            y[i] = random.gen_range(0.2f32*self::WIDTH as f32, 0.8f32 * self::HEIGHT as f32);

            vx[i] = random.gen_range(-1f32, 1f32);
            vy[i] = random.gen_range(-1f32, 1f32);
        }
        Universe {
            t: 0.0f64, x, y, m, vx, vy, ax: [0.0; N], ay: [0.0; N]
        }
    }

    pub fn width(&self) -> f32 {
        self::WIDTH as f32
    }

    pub fn height(&self) -> f32 {
        self::HEIGHT as f32
    }

    pub fn num_particles(&self) -> f64 {
        self::N as f64
    }

    pub fn px(&self) -> *const f32 {
        self.x.as_ptr()
    }

    pub fn py(&self) -> *const f32 {
        self.y.as_ptr()
    }

    pub fn tick(&mut self) {
        let dt = self::DT;
        self.t += dt as f64;

        // Naive Verlet
        for i in 0..self::N {
            // Update pos
            self.x[i] += self.vx[i] * dt + self.ax[i] * dt * dt *0.5;
            self.y[i] += self.vy[i] * dt + self.ay[i] * dt * dt *0.5;

            // Previous acceleration
            let acc_x = self.ax[i];
            let acc_y = self.ay[i];

            // Update acceleration
            let mi = self.m[i];
            let x1 = self.x[i];
            let y1 = self.y[i];
            self.ax[i] = 0.0;
            self.ay[i] = 0.0;
            for j in 0..self::N {
                if i == j {
                    continue;
                }
                let x2 = self.x[j];
                let y2 = self.y[j];
                let dx = x2 - x1;
                let dy = y2 - y1;
                let mut d2 = dx * dx + dy * dy;
                if d2 < 1.0 {
                    d2 = 1.0
                }
                let f = mi * self.m[j]/d2;
                self.ax[i] += f * dx;
                self.ay[i] += f * dy;
            }

            // Update velocity
            self.vx[i] += (acc_x + self.ax[i]) * 0.5f32 * dt;
            self.vy[i] += (acc_y + self.ay[i]) * 0.5f32 * dt;
        }
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}