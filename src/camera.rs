extern crate nalgebra;
extern crate piston_window;

pub type ScreenPoint = nalgebra::Point2<u32>;
pub type ScreenVector = nalgebra::Vector2<u32>;
pub type WorldPoint = nalgebra::Point2<f64>;
pub type WorldVector = nalgebra::Vector2<f64>;

pub struct Camera2 {
  pub viewport: WorldVector,
  pub position: WorldPoint,
  pub velocity: WorldVector,
}

impl Camera2 {
  pub fn on_update(&mut self, update_args: &piston_window::UpdateArgs) {
    let new_position = self.position + self.velocity * update_args.dt;
    self.position = new_position;
  }
}
