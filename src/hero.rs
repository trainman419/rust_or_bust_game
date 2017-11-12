extern crate ai_behavior;
extern crate piston_window;
extern crate sprite;
extern crate uuid;

use self::ai_behavior::{
    Action,
    Sequence,
    WaitForever,
    While,
};

use std::cell::RefCell;
use std::rc::Rc;

use assets;
use entity;
use error;
use level;

type Texture = piston_window::G2dTexture;
type SceneRcRef = Rc<RefCell<sprite::Scene<Texture>>>;


pub struct Hero {
  name: String,
  image: String,
  position: entity::WorldPoint2,
  velocity: entity::WorldVector2,
  scale: f64,
  visible: bool,
  active: bool,
  sprite_id: uuid::Uuid,
  scene: SceneRcRef,
}


impl Hero {
  pub fn new(
    actor: &level::Actor,
    assets: &assets::AssetMap,
    scene: SceneRcRef,
  ) -> Hero {
    let hero_texture = assets.get(&actor.image)
        .expect("Could not find asset")
        .frames.get(0).unwrap().texture.clone();

    let mut hero_sprite = sprite::Sprite::from_texture(hero_texture);

    hero_sprite.set_position(actor.position.x, actor.position.y);
    hero_sprite.set_scale(actor.scale, actor.scale);

    let hero_id: uuid::Uuid = scene.borrow_mut().add_child(hero_sprite);

    let seq = Sequence(vec![
        While(Box::new(WaitForever), vec![
              Action(sprite::Ease(sprite::EaseFunction::ExponentialIn, Box::new(sprite::MoveBy(3.0, 0.0, 50.0)))),
              Action(sprite::Ease(sprite::EaseFunction::ExponentialIn, Box::new(sprite::MoveBy(3.0, 0.0, -50.0)))),
        ]),
        ]);
    scene.borrow_mut().run(hero_id, &seq);

    Hero {
      name: actor.name.to_owned(),
      image: actor.image.to_owned(),
      position: entity::WorldPoint2::new(actor.position.x, actor.position.y),
      velocity: entity::WorldVector2::new(0.0, 0.0),
      scale: actor.scale,
      visible: actor.visible,
      active: actor.active,
      sprite_id: hero_id,
      scene: scene,
    }
  }
}


impl entity::Actor for Hero {
  fn name(&self) -> String {
    self.name.clone()
  }

  fn image(&self) -> String {
    self.image.clone()
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

  fn on_update(&mut self, update_args: &piston_window::UpdateArgs) {
    let new_position = self.position + self.velocity * update_args.dt;
    self.position = new_position;
  }

  fn interact_hero(&mut self) {
    println!("Hero interacted with Hero!");
  }

  fn interact_detective(&mut self) {
    println!("Hero interacted with Detective!");
  }
}
