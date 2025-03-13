use macroquad::prelude::*;
mod player;
mod block;
mod ball;

use player::Player;
use block::{Block, BLOCK_SIZE};
use ball::Ball;

#[macroquad::main("Breakout")]
async fn main() {
    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();

    let (width, height) = (6, 6);
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let block_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5f32, 50f32);

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(block_start_pos + vec2(block_x, block_y)));
    }

    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));

    loop {
        player.update(get_frame_time());
        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }
        clear_background(WHITE);
        player.draw();
        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter() {
            ball.draw();
        }
        next_frame().await
    }
}