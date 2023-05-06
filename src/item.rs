use macroquad::prelude::*;

use super::BALL_SPEED;
use super::actor;

pub(crate) struct Ball {
    pub(crate) pos: Vec2,
    pub(crate) sye: Vec2,
    pub(crate) vel: Vec2,
}

impl Ball {
    pub(crate) fn new(
        pos: Vec2,
        sye: Vec2,
        vel: Vec2,
    ) -> Self {
        Self {
            pos,
            sye,
            vel,
        }
    }

    pub(crate) fn update(&mut self, player: &actor::Player) {
        self.pos += self.vel * BALL_SPEED;
        if self.pos.x < 0.0 || self.pos.x > screen_width() {
            self.vel.x *= -1.0;
        }
        if self.pos.y < 0.0 || self.pos.y > screen_height() {
            self.vel.y *= -1.0;
        }

        self.collision(player);
    }

    pub(crate) fn collision(&mut self, player: &actor::Player) {
        if self.pos.x < player.pos.x + player.sye.x &&
            self.pos.x + self.sye.x > player.pos.x &&
            self.pos.y < player.pos.y + player.sye.y &&
            self.pos.y + self.sye.y > player.pos.y
        {
            self.vel.y *= -1.0;
        }
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