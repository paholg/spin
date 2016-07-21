//! This is a module comment!

// #![feature(plugin)]
// #![no_std]
// #![plugin(macro_zinc)]

#[macro_use]
extern crate glium;

extern crate typenum;
extern crate generic_array;
// extern crate zinc;

pub mod sim;
pub use sim::Spin;

pub mod color;

pub const NLEDS: usize = 16;
