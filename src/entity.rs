extern crate uuid;

use std::rc::Rc;
use std::collections::HashMap;

use error;

pub type EntityMap = HashMap<String, Rc<Actor>>;

pub trait Actor {
  fn name(&self) -> String;
  fn image(&self) -> String;
  fn position(&self) -> (f64, f64);
  fn scale(&self) -> f64;
  fn visible(&self) -> bool;
  fn active(&self) -> bool;
  fn sprite_id(&self) -> uuid::Uuid;

  fn set_position(&mut self, position: (f64, f64)) -> error::Result<()>;
  fn set_scale(&mut self, scale: f64) -> error::Result<()>;
  fn set_visible(&mut self, visible: bool) -> error::Result<()>;
  fn set_active(&mut self, active: bool) -> error::Result<()>;

  fn interact_hero(&mut self) {
    // What happens when this object interacts with the hero (i.e. ghost)
  }

  fn interact_detective(&mut self) {
    // What happens when this object interacts with the detective
  }
}
