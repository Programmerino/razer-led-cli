mod board;
mod driver_sysfs;

use std::io::{self, BufRead};
fn main() {
    let mut k = board::KeyboardData::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let [r, g, b] = parse_hex_color(&line.unwrap());
        k.set_kbd_colour(r, g, b);
        k.update_kbd();
    }
}

fn parse_hex_color(hex_color: &str) -> [u8; 3] {
    let z = i64::from_str_radix(hex_color.trim_start_matches('#'), 16).unwrap();
    [
        ((z >> 16) & 0xff) as u8,
        ((z >> 8) & 0xff) as u8,
        (z & 0xff) as u8,
    ]
}
