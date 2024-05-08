use macroquad::color::Color;
use macroquad::math::clamp;
use macroquad::rand;
use macroquad::shapes::draw_line;

/// Star struct:  in 2D position (x,y)
/// z: is used to calculate the translation (speed)
/// pz: is the previous z value, used to draw a line between these points.
pub(crate) struct Star {
    x: f32,
    y: f32,
    z: f32,
    pz: f32,
    col: (f32, f32, f32),
}

impl Star {
    pub(crate) fn new() -> Self {
        let z = rand::gen_range(0.0, 1.0);
        Star {
            x: rand::gen_range(-1.0, 1.0),
            y: rand::gen_range(-1.0, 1.0),
            z,
            pz: z,
            col: color_palette::random_color(None),
        }
    }
    pub(crate) fn update(&mut self, speed: f32) {
        self.z -= speed;
        if self.z < 0.1 {
            self.z = 1.0;
            self.x = rand::gen_range(-1.0, 1.0);
            self.y = rand::gen_range(-1.0, 1.0);
            self.pz = self.z;
        }
    }

    pub(crate) fn show(&mut self) {
        let sx = clamp(self.x / self.z, -1.1, 1.1);
        let sy = clamp(self.y / self.z, -1.1, 1.1);
        let px = clamp(self.x / self.pz, -1.1, 1.1);
        let py = clamp(self.y / self.pz, -1.1, 1.1);
        let (r, g, b) = self.col;
        draw_line(px, py, sx, sy, 0.007, Color::new(r, g, b, 1.0));

        self.pz = self.z;
    }
}

#[cfg(test)]
mod tests {
    use crate::star::Star;

    #[test]
    fn test_new_star() {
        let s = Star::new();
        assert!(s.x >= -1.0 && s.x <= 1.0);
        assert!(s.y >= -1.0 && s.y <= 1.0);
        assert!(s.z >= 0.0 && s.z <= 1.0);
        assert_eq!(s.pz, s.z);
    }
    #[test]
    fn test_update_star() {
        let mut s = Star::new();
        let p_z = s.z;
        let p_pz = s.pz;

        s.update(0.01);
        assert_ne!(p_z, s.z);
        assert_ne!(p_pz, s.pz);
    }

    #[test]
    fn test_show_star() {
        let mut s = Star::new();
        s.update(0.1);
        let p_z = s.z;
        s.show();
        assert_eq!(p_z, s.pz);
    }
}
