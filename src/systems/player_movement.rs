use ggez::GameResult;
use ggez::{Context};
use ggez::winit::event::VirtualKeyCode;

use crate::components::player::Square;
use crate::components::wall::Wall;
use crate::systems::collidable::Collidable;

pub struct PlayerMovement {
    speed: f32,
}

impl PlayerMovement {
    pub fn new(speed: f32) -> Self {
        PlayerMovement { speed }
    }

    pub fn update(&mut self, ctx: &mut Context, square: &mut Square, obstacles: &[impl Collidable]) -> GameResult {
        // Moving the square based on pressed keys
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::Up) {
            if !self.check_collision(square.x, square.y - self.speed, square.size, obstacles) {
                square.y -= self.speed;
            }
        }
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::Down) {
            if !self.check_collision(square.x, square.y + self.speed, square.size, obstacles){
                square.y += self.speed;
            }
        }
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::Left) {
            if !self.check_collision(square.x - self.speed, square.y, square.size, obstacles){
                square.x -= self.speed;
            }
        }
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::Right) {
            if !self.check_collision(square.x + self.speed, square.y, square.size, obstacles){
                square.x += self.speed;
            }
        }

        Ok(())
    }

    fn check_collision(&self, new_x: f32, new_y: f32, size: f32, obstacles: &[impl Collidable]) -> bool {
        for obstacle in obstacles {
            if obstacle.check_collision(new_x, new_y, size) {
                return true; // Colidiu com algum obstáculo
            }
        }
        false // Não colidiu com nenhum obstáculo
    }
}