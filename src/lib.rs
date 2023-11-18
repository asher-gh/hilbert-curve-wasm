mod utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HilbertCurve {
    points: Vec<u32>,
}

#[wasm_bindgen]
impl HilbertCurve {
    pub fn new(order: u8) -> Self {
        let n = 2_u32.pow(order as u32);
        let max_d = n * n;
        let mut points = Vec::<u32>::new();

        for d in 0..max_d {
            let (x, y) = d2xy(n, d);
            points.push(x);
            points.push(y);
        }

        Self { points }
    }

    pub fn points(&self) -> *const u32 {
        self.points.as_slice().as_ptr()
    }

    pub fn points_len(&self) -> u32 {
        self.points.len() as u32
    }
}

// Convert d to (x, y)
fn d2xy(n: u32, mut d: u32) -> (u32, u32) {
    let mut rx: bool;
    let mut ry: bool;
    let mut s: u32 = 1;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    while s < n {
        rx = 1 & (d / 2) == 1;
        ry = 1 & (d ^ rx as u32) == 1;
        (x, y) = rot(s, x, y, rx, ry);
        (x, y) = (x + s * rx as u32, y + s * ry as u32);
        d /= 4;
        s *= 2;
    }
    (x, y)
}

// Rotate/flip a quadrant appropriately
fn rot(n: u32, mut x: u32, mut y: u32, rx: bool, ry: bool) -> (u32, u32) {
    if !ry {
        if rx {
            x = n - 1 - x;
            y = n - 1 - y;
        }

        // Swap x and y
        std::mem::swap(&mut x, &mut y);
    }
    (x, y)
}
