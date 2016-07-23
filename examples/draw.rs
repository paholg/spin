extern crate spin;
#[macro_use]
extern crate generic_array;
extern crate typenum;

use spin::{Spin, NLEDS, LedMatrixSlice};
use spin::color::Rgb;
use spin::color::colors::*;

fn main() {
    let mut spin = Spin::new();

    let data = include!("../img.dat");
    let picture = &LedMatrixSlice::with_angles(data);
    loop {
        let phi = spin.phi();
        spin.leds = picture.get(phi);
        spin.update();
        spin.sleep_us(100);
    }
}
