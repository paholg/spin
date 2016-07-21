extern crate spin;
extern crate palette;

fn main() {
    use spin::Spin;
    let mut spinner = Spin::new();

    use palette::{Gradient, Rgb};
    let grad = Gradient::new(vec![
        Rgb::new_u8(255,   0,   0),
        Rgb::new_u8(255, 255,   0),
        Rgb::new_u8(  0, 255,   0),
        Rgb::new_u8(  0, 255, 255),
        Rgb::new_u8(  0,   0, 255),
        Rgb::new_u8(255,   0, 255),
        Rgb::new_u8(255,   0,   0),
    ]);

    use spin::NLEDS;
    let mut grad_it = grad.take(NLEDS + 1).cycle();

    loop {
        for led in spinner.leds.iter_mut() {
            let color: Rgb = Rgb::from(grad_it.next().unwrap());
            *led = spin::color::Rgb::new((color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8);
        }

        spinner.update();

        use std::time::Duration;
        ::std::thread::sleep(Duration::from_millis(2));
    }
}
