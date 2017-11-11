extern crate uuid;

use std::rc::Rc;
use std::collections::HashMap;

pub type EntityMap = HashMap<String, Rc<Actor>>;

pub trait Actor {
  fn interact_hero(&mut self) {
    // What happens when this object interacts with the hero (i.e. ghost)
  }

  fn interact_detective(&mut self) {
    // What happens when this object interacts with the detective
  }
}

pub trait Sprited {
  fn set_sprite_id(&mut self, new_id: uuid::Uuid);
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
