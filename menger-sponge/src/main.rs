use crate::sponge_box::SpongeBox;
use macroquad::camera::{set_camera, Camera2D};
use macroquad::color::Color;
use macroquad::input;
use macroquad::input::MouseButton;
use macroquad::math::{vec2, Vec3};
use macroquad::prelude::{clear_background, get_internal_gl, glam, next_frame};

const LIMIT: u8 = 4;
mod sponge_box;

/// Sketch state structure
struct Sponge {
    angle: f32,
    boxes: Vec<SpongeBox>,
    depth: u8,
}

#[macroquad::main("MengerSponge")]
async fn main() {
    let camera = Camera2D {
        zoom: vec2(1., 1.),
        target: vec2(0., 0.),
        ..Default::default()
    };

    let (r, g, b) = color_palette::BEIGE;
    let background_color = Color::new(r, g, b, 1.0);
    let (r, g, b) = color_palette::random_color(Some(vec![color_palette::BEIGE]));
    let col = Color::new(r, g, b, 0.5);
    let boxes = vec![SpongeBox::new(Vec3::new(0., 0., 0.), 0.5, col)];
    let mut sponge = Sponge {
        angle: 0.,
        boxes,
        depth: 1,
    };

    set_camera(&camera);

    // Need gl to apply rotation transformations
    let gl = unsafe { get_internal_gl().quad_gl };
    loop {
        clear_background(background_color);

        // Given exponential order, it is limited to 4 levels (8,000 boxes)
        if input::is_mouse_button_pressed(MouseButton::Left) && sponge.depth < LIMIT {
            let mut next: Vec<SpongeBox> = vec![];
            for sp in sponge.boxes.iter() {
                let new_sponges = sp.generate();
                next.extend(new_sponges)
            }

            sponge.boxes = next;
            sponge.depth += 1;
        }

        // Rotation in 3D
        gl.push_model_matrix(glam::Mat4::from_rotation_x(sponge.angle));
        gl.push_model_matrix(glam::Mat4::from_rotation_y(sponge.angle * 0.4));
        gl.push_model_matrix(glam::Mat4::from_rotation_z(sponge.angle * 0.1));

        // Drawing each box
        sponge.boxes.iter().for_each(|b| {
            b.show();
        });

        sponge.angle += 0.01;
        if sponge.angle > 360. {
            sponge.angle = 0.;
        }
        next_frame().await
    }
}
