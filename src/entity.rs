extern crate nalgebra;
extern crate piston_window;
extern crate uuid;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use error;

pub type EntityRcRef = Rc<RefCell<Actor>>;
pub type EntityMap = HashMap<String, Rc<RefCell<Actor>>>;
pub type WorldPoint2 = nalgebra::Point2<f64>;
pub type WorldVector2 = nalgebra::Vector2<f64>;

pub trait Actor {
  fn name(&self) -> String;
  fn position(&self) -> WorldPoint2;
  fn velocity(&self) -> WorldVector2;
  fn scale(&self) -> f64;
  fn visible(&self) -> bool;
  fn active(&self) -> bool;
  fn sprite_id(&self) -> uuid::Uuid;

  fn set_position(&mut self, position: WorldPoint2) -> error::Result<()>;
  fn set_velocity(&mut self, velocity: WorldVector2) -> error::Result<()>;
  fn set_scale(&mut self, scale: f64) -> error::Result<()>;
  fn set_visible(&mut self, visible: bool) -> error::Result<()>;
  fn set_active(&mut self, active: bool) -> error::Result<()>;

  fn on_update(&mut self, update_args: &piston_window::UpdateArgs) -> error::Result<()>;

  fn interact_hero(&mut self) {
    // What happens when this object interacts with the hero (i.e. ghost)
  }

  fn interact_detective(&mut self) {
    // What happens when this object interacts with the detective
  }
}
