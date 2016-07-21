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
    pub fn mix(&self, other: Rgb, factor: f32) -> Rgb {
        Rgb {
            r: factor.mul_add(other.r as f32 - self.r as f32, self.r as f32).round() as u8,
            g: factor.mul_add(other.g as f32 - self.g as f32, self.g as f32).round() as u8,
            b: factor.mul_add(other.b as f32 - self.b as f32, self.b as f32).round() as u8,
        }
    }
}


use generic_array::{GenericArray, ArrayLength};
use typenum::NonZero;

#[derive(Clone, Debug)]
pub struct Gradient<N: NonZero + ArrayLength<(f32, Rgb)>> {
    data: GenericArray<(f32, Rgb), N>,
}

impl<N> Gradient<N> where N: NonZero + ArrayLength<Rgb> + ArrayLength<(f32, Rgb)> {
    pub fn new(colors: GenericArray<Rgb, N>) ->  Gradient<N> {
        let mut points = colors.map(|&c| (0.0, c));
        let step = 1.0 / (points.len() - 1) as f32;
        for (i, point) in points.iter_mut().enumerate() {
            point.0 = i as f32 * step;
        }

        Gradient { data: points }
    }
}


// pub fn map(&self, val: u8) -> Color {
//     assert!(val <= 100, "Tried to get a color using index {} (needs to be in [0,100]).", val);
//     let mut i = 1;
//     while self.values[i] < val {
//         i += 1;
//     }
//     let lower: f32 = ((self.values[i] - val) as f32)/((self.values[i] - self.values[i-1]) as f32);
//     let upper: f32 = ((val - self.values[i-1]) as f32)/((self.values[i] - self.values[i-1]) as f32);

//     let red: u8 = (lower*(self.colors[i-1].r as f32) + upper*(self.colors[i].r as f32)) as u8;
//     let green: u8 = (lower*(self.colors[i-1].g as f32) + upper*(self.colors[i].g as f32)) as u8;
//     let blue: u8 = (lower*(self.colors[i-1].b as f32) + upper*(self.colors[i].b as f32)) as u8;
//     Color{r: red, g: green, b: blue}
// }
