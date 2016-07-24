extern crate spin;
#[macro_use]
extern crate generic_array;
extern crate typenum;

use spin::{Spin, NLEDS};
use spin::color::{Rgb, Gradient, GradientSlice};
use spin::rand::{XorShiftRng, Rng};
use spin::color::colors::*;

fn main() {
    let mut spinner = Spin::new();

    let rainbow = Gradient::new(arr![Rgb; RED, YELLOW, GREEN, CYAN, BLUE, MAGENTA]);
    let hot = Gradient::new(arr![Rgb; BLACK, RED, YELLOW, WHITE, YELLOW, RED]);
    let cool = Gradient::new(arr![Rgb; CYAN, PURPLE, MAGENTA, NAVY, TEAL]);
    let forest = Gradient::new(arr![Rgb; FOREST, CYAN, BLUE, NAVY, DKGREEN]);

    let grads = arr![&GradientSlice; &rainbow, &hot, &cool, &forest];

    let funs = [filled, bounce, in_and_out, double_tick];

    let mut rng = spin::rng();

    loop {
        let grad = grads[rng.gen_range(0, grads.len())];
        let fun = funs[rng.gen_range(0, funs.len())];
        fun(&mut spinner, &mut rng, grad);
    }
}

fn filled(spin: &mut Spin, _: &mut XorShiftRng, grad: &GradientSlice) {
    for (i, color) in grad.take(NLEDS + 1).cycle().take(50_000).enumerate() {
        let index = i % NLEDS;
        spin.leds[index] = color;

        if index == NLEDS - 1 {
            spin.update();
            spin.sleep_ms(2);
        }
    }
}

fn bounce(spin: &mut Spin, _: &mut XorShiftRng, grad: &GradientSlice) {
    for color in grad.take(100) {
        spin.leds[0] = color;
        spin.update();
        spin.sleep_ms(2);

        for i in 1..NLEDS {
            spin.leds[i-1] = BLACK;
            spin.leds[i] = color;
            spin.update();
            spin.sleep_ms(2);
        }

        for i in (0..NLEDS-1).rev() {
            spin.leds[i+1] = BLACK;
            spin.leds[i] = color;
            spin.update();
            spin.sleep_ms(2);
        }

        spin.leds[1] = BLACK;
    }
}


fn double_tick(spin: &mut Spin, _: &mut XorShiftRng, grad: &GradientSlice) {
    for color in grad.take(400) {
        spin.leds[0] = color;
        spin.leds[NLEDS-1] = color;
        spin.update();
        spin.sleep_ms(2);

        for i in 1..NLEDS/2 {
            spin.leds[i-1] = BLACK;
            spin.leds[NLEDS-i] = BLACK;

            spin.leds[i] = color;
            spin.leds[NLEDS-i-1] = color;

            spin.update();
            spin.sleep_ms(2);
        }

        spin.leds[NLEDS/2-1] = BLACK;
        spin.leds[NLEDS/2] = BLACK;
    }
}

fn in_and_out(spin: &mut Spin, rng: &mut XorShiftRng, grad: &GradientSlice) {
    let niters = 100;

    let sleep_time = 150; // us

    let (min, max) = grad.domain();

    spin.leds = [BLACK; NLEDS];
    spin.update();

    for _ in 0..4 {
        // fill in to out
        spin.sleep_us(sleep_time * niters as u32);
        for i in 1..NLEDS+1 {
            for _ in 0..niters {
                let idx = rng.gen_range(0, i);
                let blank = rng.gen();
                spin.leds[idx] = if blank { BLACK } else { grad.get(rng.gen_range(min, max)) };
                spin.update();
                spin.sleep_us(sleep_time);
            }
        }

        // wipe in to out
        for i in 0..NLEDS {
            if i > 0 {
                spin.leds[i-1] = BLACK;
            }

            for _ in 0..niters {
                let idx = rng.gen_range(i, NLEDS);
                let blank = rng.gen();
                spin.leds[idx] = if blank { BLACK } else { grad.get(rng.gen_range(min, max)) };
                spin.update();
                spin.sleep_us(sleep_time);
            }
        }
        spin.sleep_us(sleep_time * niters as u32);

        // fill out to in
        for i in (0..NLEDS).rev() {
            for _ in 0..niters {
                let idx = rng.gen_range(i, NLEDS);
                let blank = rng.gen();
                spin.leds[idx] = if blank { BLACK } else { grad.get(rng.gen_range(min, max)) };
                spin.update();
                spin.sleep_us(sleep_time);
            }
        }

        // wipe out to in
        for i in (1..NLEDS+1).rev() {
            for _ in 0..niters {
                let idx = rng.gen_range(0, i);
                let blank = rng.gen();
                spin.leds[idx] = if blank { BLACK } else { grad.get(rng.gen_range(min, max)) };
                spin.update();
                spin.sleep_us(sleep_time);
            }
            spin.leds[i-1] = BLACK;
        }
        spin.sleep_us(sleep_time * niters as u32);
    }
}
