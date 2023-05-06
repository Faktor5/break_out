use macroquad::prelude::*;

mod actor;
mod item;

const PADDLE_SPEED:f32 = 10f32;
const BALL_SPEED:f32 = 7.5f32;

fn window_conf() -> Conf {
    Conf {
        window_title: "BreakOut".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    
    rand::srand(105);
    
    let mut player = create_player();
    let mut ball = create_ball();

    loop {
        clear_background(BLACK);
        
        player.update(KeyCode::W,KeyCode::E,);
        player.draw();

        ball.update(&player);
        ball.draw();

        if ball.pos.y >= screen_height() {
            ball = create_ball();
        }

        next_frame().await
    }
}

fn create_player() -> actor::Player {
    actor::Player::new(
        Vec2::new(
            screen_width() / 2.0 - 50.0,
            screen_height() - 20.0 - 20.0,
        ),
        Vec2::new(100.0, 20.0),
    )
}

fn create_ball() -> item::Ball {
    item::Ball::new(
        Vec2::new(
            screen_width() / 2.0 - 10.0,
            screen_height() / 2.0 - 10.0,
        ),
        Vec2::new(15., 15.),
        Vec2::new(
            rand::gen_range(-0.75, 0.75),
            1.,
        )
    )
}