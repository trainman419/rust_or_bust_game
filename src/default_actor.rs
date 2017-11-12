extern crate piston_window;
extern crate sprite;
extern crate uuid;
extern crate graphics;

use std::cell::RefCell;
use std::rc::Rc;

use assets;
use entity;
use error;
use level;
use sound;

pub struct DefaultActor {
  name: String,
  position: entity::WorldPoint2,
  velocity: entity::WorldVector2,
  scale: f64,
  visible: bool,
  active: bool,
  sprite_id: uuid::Uuid,
  scene: Rc<RefCell<sprite::Scene<piston_window::G2dTexture>>>,
  asset: Rc<assets::ImageAsset>,
  animating: bool,
  frame: usize,
  next_frame: f64,
}

impl DefaultActor {
  pub fn new(
    actor: &level::Actor,
    assets: &assets::AssetMap,
    scene: Rc<RefCell<sprite::Scene<piston_window::G2dTexture>>>,
  ) -> DefaultActor {
    let asset = assets.get(&actor.image)
        .expect("Could not find asset").clone();
    let texture = asset.frames.get(0).unwrap().texture.clone();

    let mut sprite = sprite::Sprite::from_texture(texture);

    sprite.set_position(actor.position.x, actor.position.y);
    sprite.set_scale(actor.scale, actor.scale);

    let id: uuid::Uuid = scene.borrow_mut().add_child(sprite);

    DefaultActor {
      name: actor.name.to_owned(),
      position: entity::WorldPoint2::new(actor.position.x, actor.position.y),
      velocity: entity::WorldVector2::new(0.0, 0.0),
      scale: actor.scale,
      visible: actor.visible,
      active: actor.active,
      sprite_id: id,
      scene: scene,
      asset: asset.clone(),
      animating: false,
      frame: 0,
      next_frame: 0.0,
    }
  }
}

impl entity::Actor for DefaultActor {
  fn name(&self) -> String {
    self.name.clone()
  }

  fn position(&self) -> entity::WorldPoint2 {
    self.position
  }

  fn velocity(&self) -> entity::WorldVector2 {
    self.velocity
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

  fn bb(&self) -> graphics::types::Rectangle {
    self.scene.borrow_mut().child_mut(self.sprite_id).unwrap().bounding_box()
  }

  fn set_position(&mut self, position: entity::WorldPoint2) -> error::Result<()> {
    self.position = position;
    if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
      sprite.set_position(self.position.x, self.position.y);
    }
    Ok(())
  }

  fn set_velocity(&mut self, velocity: entity::WorldVector2) -> error::Result<()> {
    self.velocity = velocity;
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

  fn on_update(&mut self, update_args: &piston_window::UpdateArgs) -> error::Result<()> {
    let new_position = self.position + self.velocity * update_args.dt;
    self.set_position(new_position)?;

    // update time to next frame
    if self.animating {
      self.next_frame -= update_args.dt;

      if self.next_frame <= 0.0 {
        // if it's time for the next frame, get the asset
        let asset = &self.asset;

        // get the index of the next frame
        self.frame += 1;

        // If this is the last frame, stop animation
        if self.frame + 1 >= asset.frames.len() {
          self.animating = false;
        }

        // Clamp frame number to within bounds
        if self.frame >= asset.frames.len() {
          self.frame = asset.frames.len() - 1;
        }

        //// Get the next frame
        let frame = asset.frames.get(self.frame).unwrap();

        // Set the frame time and the update the sprite
        self.next_frame += frame.frame_time;
        if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
            sprite.set_texture(frame.texture.clone());
        }
      }
    }

    Ok(())
  }

  fn interact_hero(&mut self, sounds: &mut sound::SoundEffects) {
    self.animating = true;
  }

  fn interact_detective(&mut self) {
  }
}
