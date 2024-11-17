use ggez::{Context, GameResult};
use ggez::graphics::{Color, DrawMode, Mesh, DrawParam, Canvas, Rect};

use ncollide2d::shape::{Cuboid};
use nalgebra::Vector2;
use ncollide2d::math::{Isometry, Point};
use ncollide2d::query::PointQuery;

use crate::systems::collidable::Collidable;

pub struct Wall {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
    pub shape: Cuboid<f32>,
}

impl Wall {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Wall { 
            x,
            y, 
            height, 
            width,
            shape: Cuboid::new(Vector2::new(width / 2.0, height / 2.0)),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let wall_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(self.x, self.y, self.width, self.height),
            Color::from_rgb(128, 128, 128), // Cor cinza para a parede
        )?;

        canvas.draw(&wall_mesh, DrawParam::default());
        Ok(())
    }
}

impl Collidable for Wall {
    fn check_collision(&self, x: f32, y: f32, size: f32) -> bool {
        // Verificação se o quadrado (com base na nova posição) colide com a parede
        x + size > self.x && x < self.x + self.width && y + size > self.y && y < self.y + self.height
    }
}