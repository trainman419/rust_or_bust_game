extern crate nalgebra;
extern crate piston_window;
extern crate uuid;
extern crate ncollide;
extern crate graphics;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use error;
use sound;

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

  fn bb(&self) -> graphics::types::Rectangle;

  fn set_position(&mut self, position: WorldPoint2) -> error::Result<()>;
  fn set_velocity(&mut self, velocity: WorldVector2) -> error::Result<()>;
  fn set_scale(&mut self, scale: f64) -> error::Result<()>;
  fn set_visible(&mut self, visible: bool) -> error::Result<()>;
  fn set_active(&mut self, active: bool) -> error::Result<()>;

  fn on_update(&mut self, update_args: &piston_window::UpdateArgs) -> error::Result<()>;

  fn interact_hero(&mut self, _sounds: &mut sound::SoundEffects) {
    // What happens when this object interacts with the hero (i.e. ghost)
  }

  fn interact_detective(&mut self) {
    // What happens when this object interacts with the detective
  }

  fn overlap(&self, other: &Actor) -> bool {
    let r1 = self.bb();
    let r2 = other.bb();

    let r1_xmin = r1[0];
    let r1_xmax = r1[0] + r1[2];
    let r2_xmin = r2[0];
    let r2_xmax = r2[0] + r2[2];

    if r1_xmax < r2_xmin {
      false
    } else if r2_xmax < r1_xmin {
      false
    } else {
      true
    }
  }

  fn set_velocity_x(&mut self, x: f64) {
    let mut velocity = self.velocity();
    velocity.x = x;
    self.set_velocity(velocity);
  }
}
