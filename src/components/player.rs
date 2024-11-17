use ggez::{graphics, Context, GameResult};
use ggez::graphics::{Color, DrawMode, Mesh, DrawParam, Canvas};

use ncollide2d::shape::{Cuboid};
use nalgebra::Vector2;
use ncollide2d::math::{Isometry, Point};
use ncollide2d::query::PointQuery;

use crate::systems::collidable::Collidable;

// Defining the Square structure
pub struct Square {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub shape: Cuboid<f32>,
}

// Implementing methods for the Square
impl Square {
    // Function to create a new square at the initial position
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Square { 
            x, 
            y, 
            size,
            shape: Cuboid::new(Vector2::new(size, size)),
        }
    }

    // Function to draw the square on the provided canvas
    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let square_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(self.x, self.y, self.size, self.size),
            Color::WHITE,
        )?;
        
        canvas.draw(&square_mesh, DrawParam::default());
        Ok(())
    }
}

impl Collidable for Square {
    fn check_collision(&self, x: f32, y: f32, size: f32) -> bool {
        let point = Point::new(x, y);
        self.shape.contains_point(&Isometry::identity(), &point)
    }
}