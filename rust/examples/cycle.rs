extern crate spin;
extern crate rand;

use spin::{Spin, NLEDS};
use spin::color::Rgb;
use rand::{XorShiftRng, Rng};

fn main() {
    let mut spinner = Spin::new();

    let mut rng = XorShiftRng::new_unseeded();
    loop {
        for _ in 0..8 {
            rand_vary(&mut spinner, &mut rng);
        }

        for i in 1..6 {
            for _ in 0..5 {
                rainbow_bounce(&mut spinner, 7/i, i);
            }
        }

        for i in 1..6 {
            for _ in 0..5 {
                rainbow_double_tick(&mut spinner, 20/i, i);
            }
        }
    }
}

fn rainbow_double_tick(mut spin: &mut Spin, iters: u32, wait_ms: u32) {
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new(255, 0, 0), wait_ms);
    }
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new(255, 255, 0), wait_ms);
    }
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new(0, 255, 0), wait_ms);
    }
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new(0, 255, 255), wait_ms);
    }
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new(0, 0, 255), wait_ms);
    }
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new(255, 0, 255), wait_ms);
    }
}

fn double_tick(spin: &mut Spin, color: Rgb, wait_ms: u32) {
    spin.leds[0] = color;
    spin.leds[NLEDS-1] = color;
    spin.update();
    spin.sleep_ms(wait_ms);

    for i in 1..NLEDS/2 {
        spin.leds[i-1] = Rgb::default();
        spin.leds[NLEDS-i] = Rgb::default();

        spin.leds[i] = color;
        spin.leds[NLEDS-i-1] = color;

        spin.update();
        spin.sleep_ms(wait_ms);
    }

    spin.leds[NLEDS/2-1] = Rgb::default();
    spin.leds[NLEDS/2] = Rgb::default();
}

fn rainbow_bounce(mut spin: &mut Spin, iters: u32, wait_ms: u32) {
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new(255, 0, 0), wait_ms);
    }
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new(255, 255, 0), wait_ms);
    }
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new(0, 255, 0), wait_ms);
    }
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new(0, 255, 255), wait_ms);
    }
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new(0, 0, 255), wait_ms);
    }
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new(255, 0, 255), wait_ms);
    }
}

fn bounce(spin: &mut Spin, color: Rgb, wait_ms: u32) {
    spin.leds[0] = color;
    spin.update();
    spin.sleep_ms(wait_ms);

    for i in 1..NLEDS {
        spin.leds[i-1] = Rgb::default();
        spin.leds[i] = color;
        spin.update();
        spin.sleep_ms(wait_ms);
    }

    for i in (0..NLEDS-1).rev() {
        spin.leds[i+1] = Rgb::default();
        spin.leds[i] = color;
        spin.update();
        spin.sleep_ms(wait_ms);
    }

    spin.leds[1] = Rgb::default();
}


fn rand_in_range(spin: &mut Spin, rng: &mut XorShiftRng, low: usize, high: usize) {
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
            let r: u8 = rng.gen_range(0, 2) * 255;
            let g: u8 = rng.gen_range(0, 2) * 255;
            let b: u8 = rng.gen_range(0, 2) * 255;
            spin.leds[i] = Rgb::new(r, g, b);
        }
    }
    spin.sleep_us(150);
    spin.update();
}

fn rand_vary(mut spin: &mut Spin, mut rng: &mut XorShiftRng) {
    let n_iters: u8 = 100;

    // clear
    for led in spin.leds.iter_mut() {
        *led = Rgb::default();
    }
    spin.update();

    // fill from in to out
    for high in 1..NLEDS + 1 {
        for _ in 0..n_iters {
            rand_in_range(&mut spin, &mut rng, 0, high);
        }
    }

    // unfill from in to out
    for low in 0..NLEDS {
        if low > 0 {
            spin.leds[low-1] = Rgb::default();
        }
        for _ in 0..n_iters {
            rand_in_range(&mut spin, &mut rng, low, NLEDS);
        }
    }

    // fill from out to in
    for low in (0..NLEDS-1).rev() {
        for _ in 0..n_iters {
            rand_in_range(&mut spin, &mut rng, low, NLEDS);
        }
    }

    // unfill from out to in
    for high in (0..NLEDS).rev() {
        if high < NLEDS {
            spin.leds[high] = Rgb::default();
        }
        for _ in 0..n_iters {
            rand_in_range(&mut spin, &mut rng, 0, high);
        }
    }
}
