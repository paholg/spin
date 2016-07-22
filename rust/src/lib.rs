//! This is a module comment!


#![cfg_attr(not(feature = "sim"), no_std)]
// #![cfg_attr(not(feature = "sim"), feature(core_float))]

extern crate typenum;
extern crate generic_array;
extern crate rand;

#[cfg(not(feature = "sim"))]
#[allow(private_in_public)]
pub use core as std;

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

pub use spin::{Spin, rng};

pub mod color;

pub const NLEDS: usize = 16;
