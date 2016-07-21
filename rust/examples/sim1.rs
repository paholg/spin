extern crate spinny;
extern crate palette;

fn main() {
    use spinny::Spin;
    let mut spinner = Spin::new();

    use palette::{Gradient, Rgb};
    let grad = Gradient::new(vec![
        Rgb::new(1.0, 0.0, 0.0),
        Rgb::new(1.0, 1.0, 0.0),
        Rgb::new(0.0, 1.0, 0.0),
        Rgb::new(0.0, 1.0, 1.0),
        Rgb::new(0.0, 0.0, 1.0),
        Rgb::new(1.0, 0.0, 1.0),
        Rgb::new(1.0, 0.0, 0.0),
    ]);

    use spinny::NLEDS;
    let mut grad_it = grad.take(NLEDS + 1).cycle();

    loop {
        for led in spinner.leds.iter_mut() {
            *led = Rgb::from(grad_it.next().unwrap());
        }

        spinner.update();

        use std::time::Duration;
        ::std::thread::sleep(Duration::from_millis(1));
    }
}
