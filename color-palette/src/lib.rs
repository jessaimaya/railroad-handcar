use macroquad::color::Color;
use rand::Rng;

pub const BLACK: Color = Color::new(0.0, 0.07, 0.09, 1.0);
pub const DARK_BLUE: Color = Color::new(0.0, 0.37, 0.45, 1.0);
pub const BLUE: Color = Color::new(0.039, 0.576, 0.588, 1.0);
pub const LIGHT_BLUE: Color = Color::new(0.580, 0.824, 0.741, 1.0);
pub const BEIGE: Color = Color::new(0.914, 0.847, 0.65, 1.0);
pub const YELLOW: Color = Color::new(0.933, 0.607, 0.0, 1.0);
pub const ORANGE: Color = Color::new(0.792, 0.404, 0.007, 1.0);
pub const DARK_ORANGE: Color = Color::new(0.733, 0.243, 0.012, 1.0);
pub const RED: Color = Color::new(0.682, 0.125, 0.07, 1.0);
pub const DARK_RED: Color = Color::new(0.608, 0.133, 0.149, 1.0);

pub const PALETTE: [Color;10] = [
    BLACK,
    DARK_BLUE,
    BLUE,
    LIGHT_BLUE,
    BEIGE,
    YELLOW,
    ORANGE,
    DARK_ORANGE,
    RED,
    DARK_RED,
];

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let ind = rng.gen_range(0..PALETTE.len());
    return PALETTE[ind];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_color() {
        let result = super::random_color();
        assert!(PALETTE.contains(&result), "Returned color is not in the palette");
    }
}
