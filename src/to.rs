use macroquad::prelude::*;
use super::target;
use super::actor;
use super::item;

pub(crate) fn create_targets() -> Vec<target::Target> {
    let mut targets = Vec::<target::Target>::new();
    targets.push(target::Target::new(
        Vec2::new(
            screen_width() / 2.0 - 200.,
            screen_height() / 2.0 - 200.,
        ),
        Vec2::new(80.0, 20.0),
        3,
    ));
    targets.push(target::Target::new(
        Vec2::new(
            screen_width() / 2.0 - 100.,
            screen_height() / 2.0 - 200.,
        ),
        Vec2::new(80.0, 20.0),
        2,
    ));
    targets.push(target::Target::new(
        Vec2::new(
            screen_width() / 2.0,
            screen_height() / 2.0 - 200.,
        ),
        Vec2::new(80.0, 20.0),
        3,
    ));

    targets
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
