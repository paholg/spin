extern crate spin;
#[macro_use]
extern crate generic_array;
extern crate typenum;

use spin::{Spin, NLEDS, LedMatrix};
use spin::color::Rgb;
use spin::color::colors::*;

fn main() {
    let mut spin = Spin::new();

    let picture = LedMatrix::new(arr![
        [Rgb; NLEDS];
        [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK],
        [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, RED],
        [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED],
        [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, RED],
        [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, RED, RED],
        [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, RED, RED, RED],
        [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, RED, RED, RED, RED],
        [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, RED, RED, RED, RED, RED],
        [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, RED, RED, RED, RED, RED, RED],
        [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, RED, RED, RED, RED, RED, RED, RED],
        [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED],
        [BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED],
        [BLACK, BLACK, BLACK, BLACK, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED],
        [BLACK, BLACK, BLACK, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED],
        [BLACK, BLACK, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED],
        [BLACK, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED],
        [RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED, RED]
    ]);

    loop {
        let phi = spin.phi();
        spin.leds = picture.get(phi);
        spin.update();
        spin.sleep_us(100);
    }
}
