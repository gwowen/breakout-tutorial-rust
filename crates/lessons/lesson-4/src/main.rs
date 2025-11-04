use macroquad::prelude::*;
mod player;
mod block;
mod ball;

use player::Player;
use block::{Block, BLOCK_SIZE};
use ball::{Ball, BALL_SIZE};

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

pub fn draw_title_text(text: &str) {
    draw_text_ex(
        text,
        40.0,
        40.0,
        TextParams {
            font_size: 30u16,
            color: BLACK,
            ..Default::default()
        },
    );
}


fn init_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (6, 6);
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5f32, 50f32);

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)));
    }
}

fn reset_game(
    score: &mut i32,
    player_lives: &mut i32,
    blocks: &mut Vec<Block>,
    balls: &mut Vec<Ball>,
    player: &mut Player,
) {
    *player = Player::new();
    *score = 0;
    *player_lives = 3;
    balls.clear();
    balls.push(Ball::new(vec2(
        screen_width() * 0.5f32 - BALL_SIZE * 0.5f32,
        screen_height() * 0.5f32,
    )));
    blocks.clear();
    init_blocks(blocks);
}

// aabb collision with positional corrections
fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    // early exit
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };

    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce on y
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            // bounce on x
            a.x -= to_signum.x * intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    true
}


#[macroquad::main("Breakout")]
async fn main() {
    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut player_lives = 3;

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

        match game_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Game => {
                player.update(get_frame_time());
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                }

                for ball in balls.iter_mut() {
                    resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);
                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                            block.lives -= 1;
                            if block.lives <= 0 {
                                score += 10;
                            }
                        }
                    }
                }

                let balls_len = balls.len();
                let was_last_ball = balls_len == 1;
                balls.retain(|ball| ball.rect.y < screen_height());
                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && was_last_ball {
                        player_lives -= 1;
                        balls.push(Ball::new(
                            player.rect.point()
                            + vec2(player.rect.w * 0.5f32 - BALL_SIZE * 0.5f32, -50f32)
                        ));
                        if player_lives <= 0 {
                            game_state = GameState::Dead;
                        }
                    }

                blocks.retain(|block| block.lives > 0);
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }
            }
            GameState::Dead | GameState::LevelCompleted => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    reset_game(
                        &mut score, 
                        &mut player_lives, 
                        &mut blocks, 
                        &mut balls, 
                        &mut player
                    );
                }
            }
        }

        clear_background(WHITE);
        player.draw();
        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter() {
            ball.draw();
        }

        match game_state {
            GameState::Menu => {
                draw_title_text("Press SPACE to Start");
            },
            GameState::Game => {
                let score_text = format!("Score: {}", score);
                draw_text_ex(
                    &score_text,
                    40.0,
                    40.0,
                    TextParams {
                        font_size: 30u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );

                let player_lives_text = format!("Lives: {}", player_lives);
                draw_text_ex(
                    &player_lives_text,
                    screen_width() / 2.0,
                    40.0,
                    TextParams {
                        font_size: 30u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            },
            GameState::LevelCompleted => {
                draw_title_text(&format!("You win! Score: {}", score));
            },
            GameState::Dead => {
                draw_title_text(&format!("You died! Score: {}", score));
            },
        }

        next_frame().await
    }
}