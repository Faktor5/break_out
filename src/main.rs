use macroquad::prelude::*;

#[macroquad::main("Breakout")]
async fn main() {
    loop {
        clear_background(BLACK);
        
        // ..
        
        next_frame().await
    }
}
