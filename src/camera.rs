extern crate nalgebra;
extern crate piston_window;

pub type ScreenPoint2 = nalgebra::Point2<u32>;
pub type ScreenVector2 = nalgebra::Vector2<u32>;
pub type WorldPoint2 = nalgebra::Point2<f64>;
pub type WorldVector2 = nalgebra::Vector2<f64>;

pub struct Camera2 {
  pub zoom: f64,
  pub position: WorldPoint2,
  pub velocity: WorldVector2,
}

impl Camera2 {
  pub fn on_update(&mut self, update_args: &piston_window::UpdateArgs) {
    let new_position = self.position + self.velocity * update_args.dt;
    self.position = new_position;
  }
}
