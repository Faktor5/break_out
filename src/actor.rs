use macroquad::prelude::*;

use super::PADDLE_SPEED;

pub(crate) struct Player {
    pub(crate) pos: Vec2,
    pub(crate) sye: Vec2,
}

impl Player {
    pub(crate) fn new(
        pos: Vec2,
        sye: Vec2,
    ) -> Self {
        Self {
            pos,
            sye,
        }
    }

    pub(crate) fn update(
        &mut self,
        left : KeyCode,
        right: KeyCode,
    ) {
        if is_key_down(left) {
            self.pos.x -= PADDLE_SPEED;
        }
        if is_key_down(right) {
            self.pos.x += PADDLE_SPEED;
        }
        self.pos.x = clamp(
            self.pos.x,
            0.0,
            screen_width() - self.sye.x
        );
    }

    pub(crate) fn draw(&self) {
        draw_rectangle(
            self.pos.x,
            self.pos.y,
            self.sye.x,
            self.sye.y,
            WHITE,
        );
    }
}
