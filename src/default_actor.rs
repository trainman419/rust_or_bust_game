extern crate piston_window;
extern crate sprite;
extern crate uuid;

use std::cell::RefCell;
use std::rc::Rc;

use assets;
use entity;
use error;
use level;

pub struct DefaultActor {
  name: String,
  image: String,
  position: (f64, f64),
  scale: f64,
  visible: bool,
  active: bool,
  sprite_id: uuid::Uuid,
  scene: Rc<RefCell<sprite::Scene<piston_window::G2dTexture>>>,
}

impl DefaultActor {
  pub fn new(
    actor: &level::Actor,
    assets: &assets::AssetMap,
    scene: Rc<RefCell<sprite::Scene<piston_window::G2dTexture>>>,
  ) -> DefaultActor {
    let texture = assets.get(&actor.image)
        .expect("Could not find asset")
        .frames.get(0).unwrap().texture.clone();

    let mut sprite = sprite::Sprite::from_texture(texture);

    sprite.set_position(actor.position.x, actor.position.y);
    sprite.set_scale(actor.scale, actor.scale);

    let id: uuid::Uuid = scene.borrow_mut().add_child(sprite);

    DefaultActor {
      name: actor.name.to_owned(),
      image: actor.image.to_owned(),
      position: (actor.position.x, actor.position.y),
      scale: actor.scale,
      visible: actor.visible,
      active: actor.active,
      sprite_id: id,
      scene: scene,
    }
  }
}

impl entity::Actor for DefaultActor {
  fn name(&self) -> String {
    self.name.clone()
  }

  fn image(&self) -> String {
    self.image.clone()
  }

  fn position(&self) -> (f64, f64) {
    self.position
  }

  fn scale(&self) -> f64 {
    self.scale
  }

  fn visible(&self) -> bool {
    self.visible
  }

  fn active(&self) -> bool {
    self.active
  }

  fn sprite_id(&self) -> uuid::Uuid {
    self.sprite_id
  }

  fn set_position(&mut self, position: (f64, f64)) -> error::Result<()> {
    self.position = position;
    if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
      sprite.set_position(self.position.0, self.position.1);
    }
    Ok(())
  }

  fn set_scale(&mut self, scale: f64) -> error::Result<()> {
    self.scale = scale;
    if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
      sprite.set_scale(self.scale, self.scale);
    }
    Ok(())
  }

  fn set_visible(&mut self, visible: bool) -> error::Result<()> {
    self.visible = visible;
    if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
      sprite.set_visible(self.visible);
    }
    Ok(())
  }

  fn set_active(&mut self, active: bool) -> error::Result<()> {
    self.active = active;
    Ok(())
  }

  fn interact_hero(&mut self) {
  }

  fn interact_detective(&mut self) {
  }
}
