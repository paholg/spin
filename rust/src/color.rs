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

/// A linear interpolation between colors.
///
/// It's used to smoothly transition between a series of colors, that can be either evenly spaced
/// or have customized positions. The gradient is continuous between the control points, but it's
/// possible to iterate over a number of evenly spaced points using the `take` method. Any point
/// outside the domain of the gradient will have the same color as the closest control point.
#[derive(Clone, Debug)]
pub struct Gradient<N: NonZero + ArrayLength<(f32, Rgb)>> {
    data: GenericArray<(f32, Rgb), N>,
}

impl<N> Gradient<N> where N: NonZero + ArrayLength<Rgb> + ArrayLength<(f32, Rgb)> {
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
    pub fn take(&self, n: usize) -> Take<N> {
        let (min, max) = self.domain();

        Take {
            grad: &self,

            num: n,
            count: 0,

            initial: min,
            step: (max - min) / n as f32,
        }
    }

    // /// Slice the gradient to limit its domain.
    // pub fn slice<R: Into<Range<f32>>>(&self, range: R) -> Slice {
    //     Slice {
    //         gradient: self,
    //         range: range.into(),
    //     }
    // }

    /// Get the limits of this gradient's domain.
    pub fn domain(&self) -> (f32, f32) {
        let len = self.data.len();
        (self.data[0].0, self.data[len - 1].0)
    }
}

#[derive(Clone, Debug)]
pub struct Take<'a, N> where N: 'a + NonZero + ArrayLength<Rgb> + ArrayLength<(f32, Rgb)> {
    grad: &'a Gradient<N>,

    num: usize,
    count: usize,

    initial: f32,
    step: f32,
}

impl<'a, N> Iterator for Take<'a, N> where N: 'a + NonZero + ArrayLength<Rgb> + ArrayLength<(f32, Rgb)> {
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

// // use core::num::Float;
// ///A domain range for gradient slices.
// #[derive(Clone, Debug, PartialEq)]
// pub struct Range<T: Float> {
//     from: Option<T>,
//     to: Option<T>,
// }

// impl<T: Float> Range<T> {
//     fn clamp(&self, mut x: T) -> T {
//         x = self.from.unwrap_or(x).max(x);
//         self.to.unwrap_or(x).min(x)
//     }

//     fn constrain(&self, other: &Range<T>) -> Range<T> {
//         if let (Some(f), Some(t)) = (other.from, self.to) {
//             if f >= t {
//                 return Range {
//                     from: self.to,
//                     to: self.to,
//                 };
//             }
//         }


//         if let (Some(t), Some(f)) = (other.to, self.from) {
//             if t <= f {
//                 return Range {
//                     from: self.from,
//                     to: self.from,
//                 };
//             }
//         }

//         Range {
//             from: match (self.from, other.from) {
//                 (Some(s), Some(o)) => Some(s.max(o)),
//                 (Some(s), None) => Some(s),
//                 (None, Some(o)) => Some(o),
//                 (None, None) => None,
//             },
//             to: match (self.to, other.to) {
//                 (Some(s), Some(o)) => Some(s.min(o)),
//                 (Some(s), None) => Some(s),
//                 (None, Some(o)) => Some(o),
//                 (None, None) => None,
//             },
//         }
//     }
// }

// impl<T: Float> From<::std::ops::Range<T>> for Range<T> {
//     fn from(range: ::std::ops::Range<T>) -> Range<T> {
//         Range {
//             from: Some(range.start),
//             to: Some(range.end),
//         }
//     }
// }

// impl<T: Float> From<::std::ops::RangeFrom<T>> for Range<T> {
//     fn from(range: ::std::ops::RangeFrom<T>) -> Range<T> {
//         Range {
//             from: Some(range.start),
//             to: None,
//         }
//     }
// }

// impl<T: Float> From<::std::ops::RangeTo<T>> for Range<T> {
//     fn from(range: ::std::ops::RangeTo<T>) -> Range<T> {
//         Range {
//             from: None,
//             to: Some(range.end),
//         }
//     }
// }

// impl<T: Float> From<::std::ops::RangeFull> for Range<T> {
//     fn from(_range: ::std::ops::RangeFull) -> Range<T> {
//         Range {
//             from: None,
//             to: None,
//         }
//     }
// }


// /// A slice of a Gradient that limits its domain.
// #[derive(Clone, Debug)]
// pub struct Slice<'a> {
//     gradient: &'a Gradient,
//     range: Range<f32>,
// }

// impl<'a> Slice<'a> {
//     /// Get a color from the gradient slice. The color of the closest domain limit will be returned
//     /// if `i` is outside the domain.
//     pub fn get(&self, i: f32) -> Rgb {
//         self.gradient.get(self.range.clamp(i))
//     }

    // ///Take `n` evenly spaced colors from the gradient slice, as an iterator.
    // pub fn take(&self, n: usize) -> Take<C> {
    //     let (min, max) = self.domain();

    //     Take {
    //         gradient: MaybeSlice::Slice(self.clone()),
    //         from: min,
    //         diff: max - min,
    //         len: n,
    //         current: 0,
    //     }
    // }

    // ///Slice this gradient slice to further limit its domain. Ranges outside
    // ///the domain will be clamped to the nearest domain limit.
    // pub fn slice<R: Into<Range<C::Scalar>>>(&self, range: R) -> Slice<C> {
    //     Slice {
    //         gradient: self.gradient,
    //         range: self.range.constrain(&range.into())
    //     }
    // }

    // ///Get the limits of this gradient slice's domain.
    // pub fn domain(&self) -> (C::Scalar, C::Scalar) {
    //     if let Range { from: Some(from), to: Some(to) } = self.range {
    //         (from, to)
    //     } else {
    //         let (from, to) = self.gradient.domain();
    //         (self.range.from.unwrap_or(from), self.range.to.unwrap_or(to))
    //     }
    // }
// }

// #[derive(Clone)]
// enum MaybeSlice<'a> {
//     NotSlice(&'a Gradient),
//     Slice(Slice),
// }

// impl<'a, C: Mix + Clone> MaybeSlice<'a, C> {
//     fn get(&self, i: C::Scalar) -> C {
//         match *self {
//             MaybeSlice::NotSlice(g) => g.get(i),
//             MaybeSlice::Slice(ref s) => s.get(i),
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct Take<'a> {
//     gradient: MaybeSlice<'a, C>,
//     from: f32,
//     diff: f32,
//     len: usize,
//     current: usize,
// }
