mod utils;
use fixedbitset::FixedBitSet;
use js_sys::Math;
use wasm_bindgen::prelude::*;

// const
// const SIZE: u8 = 4;

#[wasm_bindgen]
#[derive(Default)]
pub struct Universe {
    x: u32,
    y: u32,
    n: u32,
    s: u32,
    d: u32,
}

#[wasm_bindgen]
impl HilbertCurve {
    pub fn new(order: u32) -> Self {
        Self {}
    }

    // convert (x,y) to d
    // pub fn xy2d()
}

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }
//
// #[wasm_bindgen]
// #[repr(u8)] // represent each cel as a single byte
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum GameCell {
//     Dead = 0, // simplifies cell's live neighbour counting with addition
//     Alive = 1,
// }
//
// #[wasm_bindgen]
// pub struct Universe {
//     width: u32,
//     height: u32,
//     cells: FixedBitSet,
//     x: u8,
//     y: u8,
// }
//
// #[wasm_bindgen]
// impl Universe {
//     pub fn new() -> Self {
//         let width = SIZE;
//         let height = SIZE;
//
//         let size = (width * height) as usize;
//         let mut cells = FixedBitSet::with_capacity(size);
//
//         for i in 0..size {
//             cells.set(i, Math::random() < 0.4)
//         }
//
//         Universe {
//             width,
//             height,
//             cells,
//             x: 0,
//             y: 0,
//         }
//     }
//
//     pub fn width(&self) -> u32 {
//         self.width
//     }
//
//     pub fn height(&self) -> u32 {
//         self.height
//     }
//
//     pub fn cells(&self) -> *const u32 {
//         self.cells.as_slice().as_ptr()
//     }
//
//     pub fn x(&self) -> *const u8 {
//         &self.x
//     }
//
//     pub fn y(&self) -> *const u8 {
//         &self.y
//     }
//
//     // pub fn render(&self) -> String {
//     // self.to_string()
//     // }
//
//     fn get_index(&self, row: u32, column: u32) -> usize {
//         (row * self.width + column) as usize
//     }
//
//     fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
//         let mut count = 0;
//
//         for delta_row in [self.height - 1, 0, 1].iter().cloned() {
//             for delta_col in [self.width - 1, 0, 1].iter().cloned() {
//                 if delta_row == 0 && delta_col == 0 {
//                     continue;
//                 }
//
//                 let neighbor_row = (row + delta_row) % self.height;
//                 let neighbor_col = (column + delta_col) % self.width;
//                 let idx = self.get_index(neighbor_row, neighbor_col);
//                 count += self.cells[idx] as u8;
//             }
//         }
//
//         count
//     }
//
//     pub fn tick(&mut self) {
//         let mut next = self.cells.clone();
//
//         for row in 0..self.height {
//             for col in 0..self.width {
//                 let idx = self.get_index(row, col);
//                 let cell = self.cells[idx];
//                 next.set(
//                     idx,
//                     match (cell, self.live_neighbor_count(row, col)) {
//                         (true, x) if x < 2 => false,
//                         (true, 2 | 3) => true,
//                         (true, x) if x > 3 => false,
//                         (false, 3) => true,
//                         (otherwise, _) => otherwise,
//                     },
//                 )
//             }
//         }
//
//         self.cells = next;
//     }
// }
//
// // impl fmt::Display for Universe {
// //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// //         for line in self.cells.as_slice().chunks(self.width as usize) {
// //             for &cell in line {
// //                 let symbol = if cell == GameCell::Dead { '◻' } else { '◼' };
// //                 write!(f, "{symbol}")?;
// //             }
// //             write!(f, "\n")?;
// //         }
// //
// //         Ok(())
// //     }
// // }
//
// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {name}!"));
// }
//
// // ----------------------------- Hilbert ------------------------------
//
// #[wasm_bindgen]
// pub fn xy2d(n: u8, x: u8, y: u8) -> u8 {
//     let halving_iter =
//         std::iter::successors(Some(n / 2), |&x| if x > 0 { Some(x / 2) } else { None });
//
//     for s in halving_iter {
//         // Your code here...
//     }
//     let (rx, ry, s, d) = (false, false, 0, 0);
//     // for
//     todo!()
// }
//
// pub fn a() {
//     std::iter::successors(Some(100), |&x| {
//         if x > 0 {
//             println!("{x}");
//             Some(x / 2)
//         } else {
//             None
//         }
//     });
// }
//
// pub fn b() {
//     let mut x = 100;
//     while x > 0 {
//         println!("{x}");
//         x /= 2;
//     }
// }
