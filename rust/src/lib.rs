//! This is a module comment!


#![cfg_attr(not(feature = "sim"), no_std)]
// #![cfg_attr(not(feature = "sim"), feature(core_float))]

extern crate typenum;
extern crate generic_array;

// #[cfg(not(feature = "sim"))]
// pub use core as std;

#[macro_use]
#[cfg(feature = "sim")]
extern crate glium;

// extern crate zinc;

#[cfg(feature = "sim")]
pub mod sim;
#[cfg(feature = "sim")]
pub use sim as spin;

#[cfg(not(feature = "sim"))]
pub mod spin;
// #[cfg(not(feature = "sim"))]

pub use spin::Spin;

pub mod color;

pub const NLEDS: usize = 16;
