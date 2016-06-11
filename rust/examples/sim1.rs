extern crate spinny;
extern crate palette;

use std::time::{Instant, Duration};
use palette::{Rgb, Gradient};

use spinny::Spin;
use spinny::sim::glium::SimSpin;
use spinny::NLEDS;

fn main() {
    let mut spinner = SimSpin::new();

    let grad = Gradient::new(vec![
        Rgb::new(1.0, 0.0, 0.0),
        Rgb::new(1.0, 1.0, 0.0),
        Rgb::new(0.0, 1.0, 0.0),
        Rgb::new(0.0, 1.0, 1.0),
        Rgb::new(0.0, 0.0, 1.0),
        Rgb::new(1.0, 0.0, 1.0),
        Rgb::new(1.0, 0.0, 0.0),
        ]);
    let mut grad_it = grad.take(NLEDS + 1).cycle();

    let mut old = Instant::now();

    let fps = 60;
    let time_per_frame = Duration::new(0, 1_000_000_000 / fps);

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(old);
        if elapsed > time_per_frame {
            for led in spinner.leds() {
                *led = Rgb::from(grad_it.next().unwrap());
            }
            spinner.update();
            old = now;
        }
        else {
            std::thread::sleep(time_per_frame - elapsed);
        }
    }
}
