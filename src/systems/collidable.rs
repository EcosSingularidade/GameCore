// Trait para objetos que podem colidir
pub trait Collidable {
    fn check_collision(&self, x: f32, y: f32, size: f32) -> bool;
}
