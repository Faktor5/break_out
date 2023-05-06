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

    fn collision(&mut self, player: &actor::Player) {

        let x1 = Vec2::new(player.pos.x, player.pos.x + player.sye.x / 5.0);
        let x2 = Vec2::new(player.pos.x + player.sye.x / 5.0, player.pos.x + (player.sye.x / 5.0) * 4.0);
        let x3 = Vec2::new(player.pos.x + ((player.sye.x / 5.0) * 4.0), player.pos.x + player.sye.x);

        // is the ball on the right height
        if self.pos.y < player.pos.y + player.sye.y &&
            self.pos.y + self.sye.y > player.pos.y
        {
            // is the ball in the second quarter of the paddle
            if self.pos.x < x1.y && self.pos.x + self.sye.x > x1.x {
                self.vel.x -= 0.5;
                self.vel.y = -1.0;
            }
            // is the ball in the third quarter of the paddle
            if self.pos.x < x2.y && self.pos.x + self.sye.x > x2.x {
                self.vel.y = -1.0;
            }
            // is the ball in the fourth quarter of the paddle
            if self.pos.x < x3.y && self.pos.x + self.sye.x > x3.x {
                self.vel.x += 0.5;
                self.vel.y = -1.0;
            }
            self.vel.x = clamp(self.vel.x, -1.0, 1.0);
        }

        // if self.pos.x < player.pos.x + player.sye.x &&
        //     self.pos.x + self.sye.x > player.pos.x &&
        //     self.pos.y < player.pos.y + player.sye.y &&
        //     self.pos.y + self.sye.y > player.pos.y
        // {
        //     self.vel.y *= -1.0;
        // }
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
