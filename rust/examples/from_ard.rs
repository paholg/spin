extern crate spinny;
extern crate palette;
extern crate rand;

use std::time::Duration;
use palette::Rgb;

use spinny::Spin;
use spinny::NLEDS;

use std::thread::sleep;

fn main() {
    let mut spinner = Spin::new();

    loop {
        for _ in 0..8 {
            rand_vary(&mut spinner);
        }

        for i in 1..6 {
            for _ in 0..5 {
                rainbow_bounce(&mut spinner, 7/i, Duration::from_millis(i));
            }
        }

        for i in 1..6 {
            for _ in 0..5 {
                rainbow_double_tick(&mut spinner, 20/i, Duration::from_millis(i));
            }
        }
    }
}

fn rainbow_double_tick(mut spin: &mut Spin, iters: u64, wait: Duration) {
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new_u8(255, 0, 0), wait);
    }
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new_u8(255, 255, 0), wait);
    }
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new_u8(0, 255, 0), wait);
    }
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new_u8(0, 255, 255), wait);
    }
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new_u8(0, 0, 255), wait);
    }
    for _ in 0..iters {
        double_tick(&mut spin, Rgb::new_u8(255, 0, 255), wait);
    }
}

fn double_tick(spin: &mut Spin, color: Rgb, wait: Duration) {
    spin.leds[0] = color;
    spin.leds[NLEDS-1] = color;
    spin.update();
    sleep(wait);

    for i in 1..NLEDS/2 {
        spin.leds[i-1] = Rgb::default();
        spin.leds[NLEDS-i] = Rgb::default();

        spin.leds[i] = color;
        spin.leds[NLEDS-i-1] = color;

        spin.update();
        sleep(wait);
    }

    spin.leds[NLEDS/2-1] = Rgb::default();
    spin.leds[NLEDS/2] = Rgb::default();
}

fn rainbow_bounce(mut spin: &mut Spin, iters: u64, wait: Duration) {
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new_u8(255, 0, 0), wait);
    }
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new_u8(255, 255, 0), wait);
    }
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new_u8(0, 255, 0), wait);
    }
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new_u8(0, 255, 255), wait);
    }
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new_u8(0, 0, 255), wait);
    }
    for _ in 0..iters {
        bounce(&mut spin, Rgb::new_u8(255, 0, 255), wait);
    }
}

fn bounce(spin: &mut Spin, color: Rgb, wait: Duration) {
    spin.leds[0] = color;
    spin.update();
    sleep(wait);

    for i in 1..NLEDS {
        spin.leds[i-1] = Rgb::default();
        spin.leds[i] = color;
        spin.update();
        sleep(wait);
    }

    for i in (0..NLEDS-1).rev() {
        spin.leds[i+1] = Rgb::default();
        spin.leds[i] = color;
        spin.update();
        sleep(wait);
    }

    spin.leds[1] = Rgb::default();
}


fn rand_in_range(spin: &mut Spin, low: usize, high: usize) {
    use rand::Rng;
    let mut rng = ::rand::thread_rng();
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
            spin.leds[i] = Rgb::new_u8(r, g, b);
        }
    }
    sleep(Duration::new(0, 150_000));
    spin.update();
}

fn rand_vary(mut spin: &mut Spin) {
    let n_iters: u8 = 100;

    // clear
    for led in spin.leds.iter_mut() {
        *led = Rgb::default();
    }
    spin.update();

    // fill from in to out
    for high in 1..NLEDS + 1 {
        for _ in 0..n_iters {
            rand_in_range(&mut spin, 0, high);
        }
    }

    // unfill from in to out
    for low in 0..NLEDS {
        if low > 0 {
            spin.leds[low-1] = Rgb::default();
        }
        for _ in 0..n_iters {
            rand_in_range(&mut spin, low, NLEDS);
        }
    }

    // fill from out to in
    for low in (0..NLEDS-1).rev() {
        for _ in 0..n_iters {
            rand_in_range(&mut spin, low, NLEDS);
        }
    }

    // unfill from out to in
    for high in (0..NLEDS).rev() {
        if high < NLEDS {
            spin.leds[high] = Rgb::default();
        }
        for _ in 0..n_iters {
            rand_in_range(&mut spin, 0, high);
        }
    }
}
