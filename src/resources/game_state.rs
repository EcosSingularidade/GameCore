use ggez::event::{EventHandler};
use ggez::graphics::{Color, Canvas, Rect};
use ggez::{Context, GameResult};

use crate::components::player::Square;
use crate::components::wall::Wall;
use crate::systems::player_movement::PlayerMovement;
use crate::systems::collidable::Collidable;

// Main game structure
pub struct Game {
    pub square: Square,
    pub player_movement: PlayerMovement,
    pub camera_x: f32,
    pub camera_y: f32,
    pub walls: Vec<Wall>, // Usando um vetor de objetos genéricos que implementam Collidable
}

// Implementing game logic
impl Game {
    pub fn new() -> Self {
        let player_movement = PlayerMovement::new(5.0);

        // Criando algumas paredes no cenário
        let walls = vec![
            Wall::new(200.0, 200.0, 100.0, 20.0), // Paredes representadas por retângulos
            Wall::new(400.0, 300.0, 100.0, 20.0),
            Wall::new(600.0, 100.0, 100.0, 20.0),
        ];

        Game {
            square: Square::new(100.0, 100.0, 50.0),
            player_movement,
            camera_x: 0.0,
            camera_y: 0.0,
            walls,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let _ = self.player_movement.update(ctx, &mut self.square, &self.walls);

        // Ajustando a posição da câmera
        self.camera_x = self.square.x - 400.0; // Centralizando o jogador horizontalmente
        self.camera_y = self.square.y - 300.0; // Centralizando o jogador verticalmente

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Criando a tela de fundo com cor clara
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(200, 200, 255)); // Cor do fundo azul claro
        
        // Ajustando as coordenadas da câmera
        let screen_rect = Rect::new(self.camera_x, self.camera_y, 800.0, 600.0);
        canvas.set_screen_coordinates(screen_rect); // Ajustando a área visível para a câmera

        // Desenhando as paredes
        for wall in &self.walls {
            // Chamando o método draw de Wall para desenhar cada parede
            wall.draw(&mut canvas, ctx)?;
        }

        // Desenhando o quadrado (player)
        self.square.draw(&mut canvas, ctx)?;

        // Finalizando o desenho e apresentando a tela
        canvas.finish(ctx)?;
        Ok(())
    }
}
