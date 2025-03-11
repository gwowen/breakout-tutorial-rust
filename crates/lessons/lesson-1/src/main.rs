use macroquad::prelude::*;

#[macroquad::main("Hello Macroquad")]
async fn main() {
    loop {
        clear_background(WHITE);

        // Draw some text to verify everything's working
        draw_text("Hello, Macroquad!", 10.0, 50.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

