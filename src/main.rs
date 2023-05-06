use macroquad::prelude::*;

mod actor;
mod item;

const PADDLE_SPEED:f32 = 10f32;
const BALL_SPEED:f32 = 7.5f32;

#[macroquad::main("Breakout")]
async fn main() {
    let mut player = actor::Player::new(
        Vec2::new(
            screen_width() / 2.0 - 50.0,
            screen_height() - 20.0 - 20.0,
        ),
        Vec2::new(100.0, 20.0),
    );

    let mut ball = item::Ball::new(
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
            ball = item::Ball::new(
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