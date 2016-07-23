extern crate spin;
#[macro_use]
extern crate generic_array;
extern crate typenum;

use spin::{Spin, LedMatrixSlice};
use spin::color::Rgb;

fn main() {
    let mut spin = Spin::new();

    let data = include!("picture.dat");
    let picture = &LedMatrixSlice::with_angles(data);

    loop {
        let phi = spin.phi();
        spin.leds = picture[phi];
        spin.update();
    }
}
