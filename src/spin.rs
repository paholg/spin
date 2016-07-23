use NLEDS;
use color::Rgb;

pub struct Spin {
    pub leds: [Rgb; NLEDS],
}

impl Spin {
    pub fn new() -> Spin {
        Spin { leds: [Rgb::default(); NLEDS] }
    }

    pub fn update(&mut self) {
    }

    pub fn phi(&self) -> f32 {
        0.0
    }

    pub fn omega(&self) -> f32 {
        0.0
    }

    pub fn alpha(&self) -> f32 {
        0.0
    }

    pub fn sleep_us(&mut self, us: u32) {
        let _ = us;
    }

    pub fn sleep_ms(&mut self, ms: u32) {
        let _ = ms;
    }

    pub fn sleep_s(&mut self, s: u32) {
        let _ = s;
    }
}


use rand::XorShiftRng;
pub fn rng() -> XorShiftRng {
    XorShiftRng::new_unseeded()
}

