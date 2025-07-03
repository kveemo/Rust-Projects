use std::io;

fn main() {
    println!("Hello, world!");
}

//Mode 0 = F to C, Mode 1 = C to F
fn convert(mode: u8, value: f32) -> f32 {
    if mode == 0 {
        (value - 30.0)/2.0
    } else {
        (value*2.0)+30.0
    }
} 