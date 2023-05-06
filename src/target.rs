use macroquad::prelude::*;

pub(crate) struct Target {
    pub(crate) pos: Vec2,
    pub(crate) sye: Vec2,
    hit: u32,
    cnt: u32,
}

impl Target {
    pub(crate) fn new(
        pos: Vec2,
        sye: Vec2,
        hit: u32,
    ) -> Self {
        Self {
            pos,
            sye,
            hit,
            cnt: 0,
        }
    }

    pub(crate) fn draw(&self) {
        draw_rectangle(
            self.pos.x,
            self.pos.y,
            self.sye.x,
            self.sye.y,
            if self.cnt == 0 { GREEN } else { RED },
        );
    }

    pub(crate) fn is_alive(&self) -> bool {
        self.cnt <= self.hit
    }

    pub(crate) fn update(&mut self) {
        self.cnt += 1;
    }
}