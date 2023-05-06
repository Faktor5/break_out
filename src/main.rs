use macroquad::prelude::*;

mod actor;
mod item;
mod target;
mod to;

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
    
    let mut player = to::create_player();
    let mut ball = to::create_ball();
    let mut targets = to::create_targets();

    loop {
        clear_background(BLACK);
        
        player.update(KeyCode::W,KeyCode::E,);
        player.draw();

        ball.update(&player, &mut targets);
        ball.draw();

        if ball.pos.y >= screen_height() {
            ball = to::create_ball();
            targets = to::create_targets();
        }

        targets.retain(|t| t.is_alive());
        targets.iter_mut().for_each(|t| t.draw());

        next_frame().await
    }
}