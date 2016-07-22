extern crate spin;
extern crate rand;
#[macro_use]
extern crate generic_array;
extern crate typenum;

use spin::{Spin, NLEDS};
use spin::color::{Rgb, Gradient, Len};
use rand::{XorShiftRng, Rng};

const WHITE:   Rgb = Rgb { r: 255, g: 255, b: 255 };
const BLACK:   Rgb = Rgb { r:   0, g:   0, b:   0 };
const RED:     Rgb = Rgb { r: 255, g:   0, b:   0 };
const GREEN:   Rgb = Rgb { r:   0, g: 255, b:   0 };
const BLUE:    Rgb = Rgb { r:   0, g:   0, b: 255 };
const YELLOW:  Rgb = Rgb { r: 255, g: 255, b:   0 };
const CYAN:    Rgb = Rgb { r:   0, g: 255, b: 255 };
const MAGENTA: Rgb = Rgb { r: 255, g:   0, b: 255 };
const ORANGE:  Rgb = Rgb { r: 255, g: 128, b:   0 };
const PURPLE:  Rgb = Rgb { r: 128, g:   0, b: 128 };
const DKGREEN: Rgb = Rgb { r:   0, g: 128, b:   0 };
const FOREST:  Rgb = Rgb { r:  34, g: 139, b:  34 };
const NAVY:    Rgb = Rgb { r:   0, g:   0, b: 128 };
const TEAL:    Rgb = Rgb { r:   0, g: 128, b: 128 };

fn main() {
    let mut spinner = Spin::new();

    let rainbow = Gradient::new(arr![Rgb; RED, YELLOW, GREEN, CYAN, BLUE, MAGENTA]);
    let hot = Gradient::new(arr![Rgb; BLACK, RED, YELLOW, WHITE, YELLOW, RED]);
    let cool = Gradient::new(arr![Rgb; CYAN, PURPLE, MAGENTA, NAVY, PURPLE, TEAL]);
    let forest = Gradient::new(arr![Rgb; FOREST, CYAN, BLUE, NAVY, DKGREEN, FOREST]);

    let grads = [&rainbow, &hot, &cool, &forest];

    let funs = [filled, bounce, rand_vary, double_tick];

    let mut rng = spin::rng();

    loop {
        // rand_vary(&mut spinner, &mut rng, &forest);
        let grad = grads[rng.gen_range(0, grads.len())];
        let fun = funs[rng.gen_range(0, funs.len())];
        fun(&mut spinner, &mut rng, grad);
    }
}

fn filled<N: Len>(spin: &mut Spin, _: &mut XorShiftRng, grad: &Gradient<N>) {
    for (i, color) in grad.take(NLEDS + 1).cycle().take(50_000).enumerate() {
        let index = i % NLEDS;
        spin.leds[index] = color;

        if index == 0 {
            spin.update();
            spin.sleep_ms(2);
        }
    }
}

fn bounce<N: Len>(spin: &mut Spin, _: &mut XorShiftRng, grad: &Gradient<N>) {
    for color in grad.take(100) {
        spin.leds[0] = color;
        spin.update();
        spin.sleep_ms(2);

        for i in 1..NLEDS {
            spin.leds[i-1] = Rgb::default();
            spin.leds[i] = color;
            spin.update();
            spin.sleep_ms(2);
        }

        for i in (0..NLEDS-1).rev() {
            spin.leds[i+1] = Rgb::default();
            spin.leds[i] = color;
            spin.update();
            spin.sleep_ms(2);
        }

        spin.leds[1] = Rgb::default();
    }
}


fn double_tick<N: Len>(spin: &mut Spin, _: &mut XorShiftRng, grad: &Gradient<N>) {
    for color in grad.take(400) {
        spin.leds[0] = color;
        spin.leds[NLEDS-1] = color;
        spin.update();
        spin.sleep_ms(2);

        for i in 1..NLEDS/2 {
            spin.leds[i-1] = Rgb::default();
            spin.leds[NLEDS-i] = Rgb::default();

            spin.leds[i] = color;
            spin.leds[NLEDS-i-1] = color;

            spin.update();
            spin.sleep_ms(2);
        }

        spin.leds[NLEDS/2-1] = Rgb::default();
        spin.leds[NLEDS/2] = Rgb::default();
    }
}


fn rand_in_range<N: Len>(spin: &mut Spin, rng: &mut XorShiftRng, grad: &Gradient<N>, low: usize, high: usize) {
    let i: usize = if low >= high {
        low
    } else {
        rng.gen_range(low, high)
    };

    let blank: bool = rng.gen();

    {
        if blank {
            spin.leds[i] = Rgb::default();
        }
        else {
            let (min, max) = grad.domain();
            spin.leds[i] = grad.get(rng.gen_range(min, max));
        }
    }
    spin.sleep_us(150);
    spin.update();
}

fn rand_vary<N: Len>(mut spin: &mut Spin, mut rng: &mut XorShiftRng, grad: &Gradient<N>) {
    let n_iters: u8 = 100;

    for _ in 0..4 {
        // clear
        for led in spin.leds.iter_mut() {
            *led = Rgb::default();
        }
        spin.update();

        // fill from in to out
        for high in 1..NLEDS + 1 {
            for _ in 0..n_iters {
                rand_in_range(&mut spin, &mut rng, &grad, 0, high);
            }
        }

        // unfill from in to out
        for low in 0..NLEDS {
            if low > 0 {
                spin.leds[low-1] = Rgb::default();
            }
            for _ in 0..n_iters {
                rand_in_range(&mut spin, &mut rng, &grad, low, NLEDS);
            }
        }

        // fill from out to in
        for low in (0..NLEDS-1).rev() {
            for _ in 0..n_iters {
                rand_in_range(&mut spin, &mut rng, &grad, low, NLEDS);
            }
        }

        // unfill from out to in
        for high in (0..NLEDS).rev() {
            if high < NLEDS {
                spin.leds[high] = Rgb::default();
            }
            for _ in 0..n_iters {
                rand_in_range(&mut spin, &mut rng, &grad, 0, high);
            }
        }
    }
}
