use macroquad::rand::*;
pub const BLACK: (f32, f32, f32) = (0.0, 0.07, 0.09);
pub const DARK_BLUE: (f32, f32, f32) = (0.0, 0.37, 0.45);
pub const BLUE: (f32, f32, f32) = (0.039, 0.576, 0.588);
pub const LIGHT_BLUE: (f32, f32, f32) = (0.580, 0.824, 0.741);
pub const BEIGE: (f32, f32, f32) = (0.914, 0.847, 0.65);
pub const YELLOW: (f32, f32, f32) = (0.933, 0.607, 0.0);
pub const ORANGE: (f32, f32, f32) = (0.792, 0.404, 0.007);
pub const DARK_ORANGE: (f32, f32, f32) = (0.733, 0.243, 0.012);
pub const RED: (f32, f32, f32) = (0.682, 0.125, 0.07);
pub const DARK_RED: (f32, f32, f32) = (0.608, 0.133, 0.149);

pub const PALETTE: [(f32, f32, f32); 10] = [
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

pub fn random_color(avoid: Option<Vec<(f32, f32, f32)>>) -> (f32, f32, f32) {
    if avoid.is_some() {
        let avoid_colors = avoid.clone().unwrap();
        let filtered: Vec<(f32, f32, f32)> = PALETTE
            .iter()
            .filter(|c| !avoid_colors.contains(c))
            .cloned()
            .collect();
        return if filtered.is_empty() {
            (0., 0., 0.)
        } else {
            let ind = gen_range(0, filtered.len());
            filtered[ind]
        };
    }
    let ind = gen_range(0, PALETTE.len());
    PALETTE[ind]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_color() {
        let result = super::random_color(None);
        assert!(
            PALETTE.contains(&result),
            "Returned color is not in the palette"
        );
    }
    #[test]
    fn avoid_random_color() {
        let mut avoid = vec![];
        for i in 0..PALETTE.len() - 1 {
            avoid.push(PALETTE[i]);
        }
        assert_eq!(avoid.len(), PALETTE.len() - 1);

        let res = super::random_color(Some(avoid));
        assert_eq!(&res, PALETTE.last().unwrap());
    }
}
