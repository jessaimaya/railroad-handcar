use macroquad::math::Vec2;

pub(crate) struct Snake {
    pos: Vec2,
    speed: Vec2,
    total: usize,
    tail: Vec<Vec2>,
}

impl Snake {
    pub(crate) fn new() -> Self {
        Snake {
            pos: Vec2::new(0., 0.),
            speed: Vec2::new(1., 0.),
            total: 0,
            tail: Vec::new(),
        }
    }

    pub(crate) fn eat(&mut self, food_pos: Vec2) -> bool {
        let dist = self.pos.distance(food_pos);
        if dist < 1. {
            self.total += 1;
            return true;
        }
        false
    }

    fn dir(&mut self, pos: Vec2) {
        self.speed = pos;
    }

    pub(crate) fn death(&mut self) {
        self.tail.iter().for_each(|pos| {
            let dist = self.pos.distance(*pos);
            if dist < 1. {
                self.total = 0;
                self.tail = Vec::new();
            }
        });
    }

    pub(crate)  fn update(&mut self, scl:f32) {
        for i in 0..self.tail.len() - 1 {
            self.tail[i] = self.tail[i+1];
        }
        if self.total >= 1 {
            self.tail[self.total-1] = self.pos;
        }
        self.pos = Vec2::new(self.pos.x + self.speed.x * scl, self.pos.y + self.speed.y * scl);
    }
}
