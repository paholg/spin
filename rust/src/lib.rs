// #![feature(plugin)]
// #![no_std]
// #![plugin(macro_zinc)]

#[macro_use]
extern crate glium;
extern crate palette;
// extern crate zinc;

use palette::Rgb;

pub mod sim;

pub const NLEDS: usize = 16;

pub trait Spin {
    fn update(&mut self);

    fn phi(&self) -> f32;
    fn omega(&self) -> f32;
    fn alpha(&self) -> f32;

    fn leds(&mut self) -> &mut [Rgb; NLEDS];
}
