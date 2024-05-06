use macroquad::color::Color;
use macroquad::math::Vec3;
use macroquad::models::draw_cube;
/// SpongeBox structure having own color
pub(crate) struct SpongeBox {
    position: Vec3,
    radius: f32,
    col: Color,
}

impl SpongeBox {
    pub(crate) fn new(position: Vec3, radius: f32, col: Color) -> Self {
        SpongeBox {
            position,
            radius,
            col,
        }
    }

    /// Need to generate a new depth level (3D) for each box
    pub(crate) fn generate(&self) -> Vec<SpongeBox> {
        let mut boxes = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    let sum: f32 = (x as f32).abs() + (y as f32).abs() + (z as f32).abs();
                    let new_radius = self.radius / 3.0;
                    if sum > 1. {
                        let pos = Vec3::new(
                            self.position.x + x as f32 * new_radius,
                            self.position.y + y as f32 * new_radius,
                            self.position.z + z as f32 * new_radius,
                        );
                        let (cr, cg, cb) =
                            color_palette::random_color(Some(vec![color_palette::BEIGE]));
                        let b = SpongeBox::new(pos, new_radius, Color::new(cr, cg, cb, 0.5));
                        boxes.push(b);
                    }
                }
            }
        }
        boxes
    }

    /// Draw each box
    pub(crate) fn show(&self) {
        draw_cube(self.position, Vec3::from([self.radius; 3]), None, self.col);
    }
}

#[cfg(test)]
mod tests {
    use crate::sponge_box::SpongeBox;
    use macroquad::color::Color;
    use macroquad::math::Vec3;

    #[test]
    fn test_sponge_box() {
        let pos = Vec3::default();
        let sponge = SpongeBox::new(pos, 0.0, Color::default());

        assert_eq!(sponge.position, pos);
    }

    #[test]
    fn test_generation() {
        let pos = Vec3::default();
        let sponge = SpongeBox::new(pos, 0.0, Color::default());
        let sponges = sponge.generate();
        assert_eq!(sponges.len(), 20);
    }
}
