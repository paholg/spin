// Note: This file is adopted from the palette library. You can view the original source
// here: https://github.com/Ogeon/palette
// The parts reproduced here have two notable differences from the original:
// First, it has been adopted to usable without the standard library.
// Second, it is far less generic. This may change in the future.

// fixme: Add the MIT licence stuff from palette, make sure I'm attributing correctly

// use typenum::Unsigned;

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r: r, g: g, b: b }
    }

    /// Mix self and other. Factor should be in the range [0.0, 1.0] where
    /// a factor of 0.0 will give a result of self and 1.0 will result in other.
    pub fn mix(&self, other: &Rgb, factor: f32) -> Rgb {
        Rgb {
            r: factor.mul_add(other.r as f32 - self.r as f32, self.r as f32).round() as u8,
            g: factor.mul_add(other.g as f32 - self.g as f32, self.g as f32).round() as u8,
            b: factor.mul_add(other.b as f32 - self.b as f32, self.b as f32).round() as u8,
        }
    }
}


use generic_array::{GenericArray, ArrayLength};
use typenum::NonZero;

pub trait Len: Clone + NonZero + ArrayLength<(f32, Rgb)> + ArrayLength<Rgb> {}
impl<T> Len for T where T: Clone + NonZero + ArrayLength<(f32, Rgb)> + ArrayLength<Rgb> {}

/// A linear interpolation between colors.
///
/// It's used to smoothly transition between a series of colors, that can be either evenly spaced
/// or have customized positions. The gradient is continuous between the control points, but it's
/// possible to iterate over a number of evenly spaced points using the `take` method. Any point
/// outside the domain of the gradient will have the same color as the closest control point.
#[derive(Clone, Debug)]
pub struct Gradient<N: Len> {
    data: GenericArray<(f32, Rgb), N>,
}

#[derive(Debug)]
pub struct GradientSlice {
    data: [(f32, Rgb)],
}

// impl GradientBorrowed {
//     pub fn new<'a, N: Len>(grad: Gradient<N>) -> &'a GradientBorrowed {
//         &GradientBorrowed { data: *grad.data }
//     }
// }

use std::ops::Deref;
impl<N: Len> Deref for Gradient<N> {
    type Target = GradientSlice;

    fn deref(&self) -> &Self::Target {
        let slice: &[(f32, Rgb)] = &self.data;
        unsafe { ::std::mem::transmute(slice) }
    }
}

impl<N> Gradient<N> where N: Len {
    /// Create a gradient of evenly spaced colors with the domain [0.0, 1.0].  There must be at
    /// least one color.
    pub fn new(colors: GenericArray<Rgb, N>) ->  Gradient<N> {
        let mut points = colors.map(|&c| (0.0, c));
        let step = 1.0 / (points.len() - 1) as f32;
        for (i, point) in points.iter_mut().enumerate() {
            point.0 = i as f32 * step;
        }

        Gradient { data: points }
    }

    /// Create a gradient of colors with custom spacing and domain. There must be at least one
    /// color and they are expected to be ordered by their position value.
    pub fn with_domain(colors: GenericArray<(f32, Rgb), N>) -> Gradient<N> {
        // fixme: ensure colors is sorted
        Gradient { data: colors }
    }
}

impl GradientSlice {
    /// Get a color from the gradient. The color of the closest control point will be returned if
    /// `i` is outside the domain.
    pub fn get(&self, i: f32) -> Rgb {
        let mut min_index = 0;
        let (mut min, ref min_color) = self.data[min_index];
        let mut min_color = min_color;

        if i <= min {
            return min_color.clone();
        }

        let mut max_index = self.data.len() - 1;
        let (mut max, ref max_color) = self.data[max_index];
        let mut max_color = max_color;

        if i >= max {
            return max_color.clone();
        }

        while min_index < max_index - 1 {
            let index = min_index + (max_index - min_index) / 2;

            let (p, ref color) = self.data[index];

            if i <= p {
                max = p;
                max_color = color;
                max_index = index;
            } else {
                min = p;
                min_color = color;
                min_index = index;
            }
        }

        let factor = (i - min) / (max - min);

        min_color.mix(max_color, factor)
    }

    /// Take `n` evenly spaced colors from the gradient, as an iterator.
    pub fn take(&self, n: usize) -> Take {
        let (min, max) = self.domain();

        Take {
            grad: &self,

            num: n,
            count: 0,

            initial: min,
            step: (max - min) / n as f32,
        }
    }

    /// Get the limits of this gradient's domain.
    pub fn domain(&self) -> (f32, f32) {
        let len = self.data.len();
        (self.data[0].0, self.data[len - 1].0)
    }
}

#[derive(Clone, Debug)]
pub struct Take<'a> {
    grad: &'a GradientSlice,

    num: usize,
    count: usize,

    initial: f32,
    step: f32,
}

impl<'a> Iterator for Take<'a> {
    type Item = Rgb;

    fn next(&mut self) -> Option<Rgb> {
        if self.count < self.num {
            let i = self.initial + self.step * self.count as f32;
            self.count += 1;
            Some(self.grad.get(i))
        } else {
            None
        }
    }
}
