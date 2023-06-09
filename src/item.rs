use macroquad::prelude::*;


use super::BALL_SPEED;
use super::target::Target;
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
    
    pub(crate) fn update(
        &mut self,
        player: &actor::Player,
        targets: &mut Vec<Target>)
    {
        
        let mut dir = self.vel.normalize();
        dir.x *= BALL_SPEED;
        dir.y *= BALL_SPEED;
        self.pos.x += dir.x;
        self.pos.y += dir.y;

        if self.pos.x < 0.0 || self.pos.x > screen_width() {
            self.vel.x *= -1.0;
        }
        if self.pos.y < 0.0 || self.pos.y > screen_height() {
            self.vel.y *= -1.0;
        }

        self.collision_player(player);
        self.collision_targets(targets);
    }

    fn collision_player(&mut self, player: &actor::Player) {

        // let x1 = Vec2::new(player.pos.x, player.pos.x + player.sye.x / 5.0);
        // let x2 = Vec2::new(player.pos.x + player.sye.x / 5.0, player.pos.x + (player.sye.x / 5.0) * 4.0);
        // let x3 = Vec2::new(player.pos.x + ((player.sye.x / 5.0) * 4.0), player.pos.x + player.sye.x);

        if self.pos.y + self.sye.y > player.pos.y && self.pos.y + self.sye.y < player.pos.y + player.sye.y
        {

            if
                self.pos.x > player.pos.x && self.pos.x < player.pos.x + player.sye.x
             || self.pos.x + self.sye.x > player.pos.x && self.pos.x + self.sye.x < player.pos.x + player.sye.x
            {
                self.vel.y *= -1.0;
            }

            // EDGE CONTROL, done for a later date
            // if self.pos.x < x1.y && self.pos.x + self.sye.x > x1.x {
            //     self.vel.x = -1.0;
            //     self.vel.y *= -1.0;
            // }

            // if self.pos.x < x2.y && self.pos.x + self.sye.x > x2.x {
            //     self.vel.y *= -1.0;
            // }

            // if self.pos.x < x3.y && self.pos.x + self.sye.x > x3.x {
            //     self.vel.x = 1.0;
            //     self.vel.y *= -1.0;
            // }

            self.vel.x = clamp(self.vel.x, -1.0, 1.0);

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

    fn collision_targets(&mut self, targets: &mut Vec<Target>) {
        for target in targets.iter_mut() {
            if
                self.pos.x > target.pos.x && self.pos.x < target.pos.x + target.sye.x
             || self.pos.x + self.sye.x > target.pos.x && self.pos.x + self.sye.x < target.pos.x + target.sye.x
            {
                if
                    self.pos.y > target.pos.y && self.pos.y < target.pos.y + target.sye.y
                 || self.pos.y + self.sye.y > target.pos.y && self.pos.y + self.sye.y < target.pos.y + target.sye.y
                {
                    target.update();
                    self.vel.y *= -1.0;
                }
            }
        }
    }
}
