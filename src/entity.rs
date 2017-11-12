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

  fn set_name(&mut self, name: String) -> error::Result<()>;
  fn set_image(&mut self, image: String) -> error::Result<()>;
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

pub trait Sprited {
  fn get_sprite_id(&self) -> uuid::Uuid;
}

pub trait Position {
  fn x(&self) -> f64;
  fn y(&self) -> f64;
  fn set_x(&mut self, new_x: f64);
  fn set_y(&mut self, new_y: f64);
  fn set_position(&mut self, new_x: f64, new_y: f64);
}

pub trait Scaled {
  fn set_scale(&mut self, new_scale: f64);
  fn get_scale(&self) -> f64;
}
