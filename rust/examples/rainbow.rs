extern crate spin;

#[macro_use]
extern crate generic_array;
extern crate typenum;
// extern crate palette;

fn main() {
    use spin::Spin;
    let mut spinner = Spin::new();

    use spin::color::{Gradient, Rgb};
    let grad = Gradient::new(arr![Rgb;
        Rgb::new(255,   0,   0),
        Rgb::new(255, 255,   0),
        Rgb::new(  0, 255,   0),
        Rgb::new(  0, 255, 255),
        Rgb::new(  0,   0, 255),
        Rgb::new(255,   0, 255),
        Rgb::new(255,   0,   0)
    ]);

    use spin::NLEDS;
    let mut grad_it = grad.take(NLEDS + 1).cycle();

    loop {
        for led in spinner.leds.iter_mut() {
            *led = grad_it.next().unwrap();
        }

        spinner.update();

        spinner.sleep_ms(2);
    }
}
