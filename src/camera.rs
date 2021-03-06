extern crate nalgebra;
extern crate piston_window;

pub type WorldPoint2 = nalgebra::Point2<f64>;
pub type WorldVector2 = nalgebra::Vector2<f64>;

pub struct Camera2 {
  pub zoom: f64,
  pub position: WorldPoint2,
  pub velocity: WorldVector2,
}

impl Camera2 {
  pub fn new() -> Camera2 {
    Camera2 {
      zoom: 1.0,
      position: WorldPoint2::new(0.0, 0.0),
      velocity: WorldVector2::new(0.0, 0.0),
    }
  }
}
