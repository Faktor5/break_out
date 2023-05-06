use macroquad::prelude::*;
use super::target;
use super::actor;
use super::item;

pub(crate) fn create_targets() -> Vec<target::Target> {
    let mut targets = Vec::<target::Target>::new();
    
    let screen_width = screen_width();
    let screen_height = screen_height();

    // create targets like a grid of 10x5 targets
    for y in 0..5 {
        for x in 0..10 {
            let pos = Vec2::new(
                (screen_width / 10.0) * x as f32 + 5.0,
                (screen_height / 10.0) * y as f32 + 5.0,
            );
            let sye = Vec2::new(
                screen_width / 10.0 - 10.0,
                screen_height / 10.0 - 10.0,
            );
            targets.push(target::Target::new(pos, sye, 1));
        }
    }

    return targets;
}

pub(crate) fn create_player() -> actor::Player {
    actor::Player::new(
        Vec2::new(
            screen_width() / 2.0 - 50.0,
            screen_height() - 20.0 - 20.0,
        ),
        Vec2::new(100.0, 20.0),
    )
}

pub(crate) fn create_ball() -> item::Ball {
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
