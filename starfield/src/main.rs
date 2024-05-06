mod star;

use crate::star::Star;
use macroquad::input;
use macroquad::prelude::*;

const QUANTITY: usize = 1000;

#[derive(Default)]
struct Stars {
    stars: Vec<Star>,
}

#[macroquad::main("Starfield")]
async fn main() {
    let camera = Camera2D {
        zoom: vec2(1., 1.),
        target: vec2(0., 0.),
        ..Default::default()
    };

    let mut stars = Stars::default();

    for _ in 0..QUANTITY {
        stars.stars.push(Star::new());
    }

    let (r, g, b) = color_palette::BEIGE;
    set_camera(&camera);
    loop {
        clear_background(Color::new(r, g, b, 1.0));

        stars.stars.iter_mut().for_each(|star| {
            let speed = utilities::map_num(input::mouse_position_local().x, -1.0, 1.0, 0.001, 0.1);
            star.update(speed);
            star.show();
        });
        next_frame().await
    }
}
