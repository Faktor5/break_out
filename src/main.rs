use macroquad::prelude::*;

const PADDLE_SPEED:f32 = 10f32;
const BALL_SPEED:f32 = 10f32;

#[macroquad::main("Breakout")]
async fn main() {
    let mut player = Player::new(
        Vec2::new(
            screen_width() / 2.0 - 50.0,
            screen_height() - 20.0 - 20.0,
        ),
        Vec2::new(100.0, 20.0),
    );

    let mut ball = Ball::new(
        Vec2::new(
            screen_width() / 2.0 - 10.0,
            screen_height() / 2.0 - 10.0,
        ),
        Vec2::new(20.0, 20.0),
        Vec2::new(
            // rng.gen_range(-1.0..1.),
            // rng.gen_range(-1.0..1.)
            0.0,
            1.0,
        )
    );

    loop {
        clear_background(BLACK);
        
        player.update(
            KeyCode::W,
            KeyCode::E,
        );
        player.draw();

        ball.update(&player);
        ball.draw();

        if ball.pos.y >= screen_height() {
            ball = Ball::new(
                Vec2::new(
                    screen_width() / 2.0 - 10.0,
                    screen_height() / 2.0 - 10.0,
                ),
                Vec2::new(20.0, 20.0),
                Vec2::new(
                    // rng.gen_range(-1.0..1.),
                    // rng.gen_range(-1.0..1.)
                    0.0,
                    1.0,
                )
            );
        }
        
        next_frame().await
    }
}

struct Ball {
    pos: Vec2,
    sye: Vec2,
    vel: Vec2,
}

impl Ball {
    fn new(
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
    
    fn update(&mut self, player: &Player) {
        self.pos += self.vel * BALL_SPEED;
        if self.pos.x < 0.0 || self.pos.x > screen_width() {
            self.vel.x *= -1.0;
        }
        if self.pos.y < 0.0 || self.pos.y > screen_height() {
            self.vel.y *= -1.0;
        }

        self.collision(player);
    }

    fn collision(&mut self, player: &Player) {
        if self.pos.x < player.pos.x + player.sye.x &&
            self.pos.x + self.sye.x > player.pos.x &&
            self.pos.y < player.pos.y + player.sye.y &&
            self.pos.y + self.sye.y > player.pos.y
        {
            self.vel.y *= -1.0;
        }
    }
    
    fn draw(&self) {
        draw_rectangle(
            self.pos.x,
            self.pos.y,
            self.sye.x,
            self.sye.y,
            WHITE,
        );
    }
}

struct Player {
    pos: Vec2,
    sye: Vec2,
}

impl Player {
    fn new(
        pos: Vec2,
        sye: Vec2,
    ) -> Self {
        Self {
            pos,
            sye,
        }
    }
    
    fn update(
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
    
    fn draw(&self) {
        draw_rectangle(
            self.pos.x,
            self.pos.y,
            self.sye.x,
            self.sye.y,
            WHITE,
        );
    }
}