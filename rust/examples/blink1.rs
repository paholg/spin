// #![feature(plugin, start, core_intrinsics)]
// #![no_std]
// #![plugin(macro_platformtree)]

// extern crate zinc;

// platformtree!(
//     lpc11xx@mcu {
//         clock {
//             source = "main-oscillator";
//             source_frequency = 12_000_000;
//             pll {
//                 m = 50;
//                 n = 3;
//                 divisor = 4;
//             }
//         }

//         timer {
//             timer@1 {
//                 counter = 25;
//                 divisor = 4;
//             }
//         }

//         gpio {
//             1 {
//                 led@14 { direction = "out"; }
//             }
//         }
//     }

//     os {
//         single_task {
//             loop = "run";
//             args {
//                 timer = &timer;
//                 led = &led;
//             }
//         }
//     }
//     );

// fn run(args: &pt::run_args) {
//     // use zinc::hal::pin::Gpio;
//     // use zinc::hal::timer::Timer;

//     // let led = args.led;
//     // let timer = args.timer;

//     // led.set_high();
//     // timer.wait(1);

//     // led.set_low();
//     // timer.wait(1);
// }

fn main() {}
