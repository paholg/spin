//! This is a module comment!


#![cfg_attr(not(feature = "sim"), no_std)]
#![cfg_attr(not(feature = "sim"), feature(core_float))]

extern crate typenum;
extern crate generic_array;
pub extern crate rand;

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
pub mod text;

pub const NLEDS: usize = 16;

use typenum::NonZero;
use generic_array::ArrayLength;
use color::Rgb;

pub trait Len: Clone + NonZero + ArrayLength<(f32, Rgb)> + ArrayLength<Rgb>
    + ArrayLength<(f32, [Rgb; NLEDS])> + ArrayLength<[Rgb; NLEDS]> {}
impl<T> Len for T where T: Clone + NonZero + ArrayLength<(f32, Rgb)> + ArrayLength<Rgb>
    + ArrayLength<(f32, [Rgb; NLEDS])> + ArrayLength<[Rgb; NLEDS]> {}


use generic_array::GenericArray;
pub struct LedMatrix<N: Len> {
    data: GenericArray<(f32, [Rgb; NLEDS]), N>,
}

use std::f32::consts::PI;
impl<N: Len> LedMatrix<N> {
    pub fn new(led_strips: GenericArray<[Rgb; NLEDS], N>) ->  LedMatrix<N> {
        let mut points = led_strips.map(|&l| (0.0, l));
        let step = 2.0 * PI / points.len() as f32;
        for (i, point) in points.iter_mut().enumerate() {
            point.0 = i as f32 * step;
        }

        LedMatrix { data: points }
    }

    pub fn with_angles(led_strips: GenericArray<(f32, [Rgb; NLEDS]), N>) ->  LedMatrix<N> {
        // fixme: ensure led_strips is sorted, and that angles are in [0, 2*PI)
        LedMatrix { data: led_strips }
    }
}

pub struct LedMatrixSlice {
    data: [(f32, [Rgb; NLEDS])],
}

impl LedMatrixSlice {
    pub fn with_angles(led_strips: &[(f32, [Rgb; NLEDS])]) ->  &LedMatrixSlice {
        // fixme: ensure led_strips is sorted, and that angles are in [0, 2*PI)
        unsafe { ::std::mem::transmute(led_strips) }
    }

    /// Get an LED strip from the matrix. `i` must be in [0.0, 2.0*PI).
    pub fn get(&self, i: f32) -> [Rgb; NLEDS] {
        assert!(i >= 0.0 && i < 2.0 * PI);
        let mut min_index = 0;
        let (mut min, ref min_strip) = self.data[min_index];
        let mut min_strip = min_strip;

        if i <= min {
            return min_strip.clone();
        }

        let mut max_index = self.data.len() - 1;
        let (mut max, ref max_strip) = self.data[max_index];
        let mut max_strip = max_strip;

        if i >= max {
            return max_strip.clone();
        }

        while min_index < max_index - 1 {
            let index = min_index + (max_index - min_index) / 2;

            let (p, ref strip) = self.data[index];

            if i <= p {
                max = p;
                max_strip = strip;
                max_index = index;
            } else {
                min = p;
                min_strip = strip;
                min_index = index;
            }
        }

        let factor = (i - min) / (max - min);

        // fixme: switch to zip once GenericArray has it
        unsafe {
            let mut res: [Rgb; NLEDS] = ::std::mem::uninitialized();
            let colors = min_strip.iter()
                .zip(max_strip.iter())
                .map(|(min_color, max_color)| min_color.mix(max_color, factor));
            for (r, color) in res.iter_mut().zip(colors) {
                ::std::ptr::write(r, color);
            }
            res
        }
    }
}

use std::ops::Deref;
impl<N: Len> Deref for LedMatrix<N> {
    type Target = LedMatrixSlice;

    fn deref(&self) -> &Self::Target {
        let slice: &[(f32, [Rgb; NLEDS])] = &self.data;
        unsafe { ::std::mem::transmute(slice) }
    }
}
