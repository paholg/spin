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

#[derive(Clone, Debug)]
pub struct Gradient<N: ArrayLength<Rgb>> {
    colors: GenericArray<Rgb, N>,
}

impl<N: ArrayLength<Rgb>> Gradient<N> {
    pub fn new(colors: GenericArray<Rgb, N>) -> Gradient<N> {
        Gradient { colors: colors }
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
