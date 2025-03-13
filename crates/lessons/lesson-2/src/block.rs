use macroquad::prelude::*;

pub const BLOCK_SIZE: Vec2 = Vec2::from_array([100f32, 40f32]);

pub struct Block {
    pub rect: Rect,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GRAY);
    }
}