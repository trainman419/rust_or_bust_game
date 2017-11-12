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
use std::collections::HashMap;

use assets;
use entity;
use error;
use level;

type Texture = piston_window::G2dTexture;
type SceneRcRef = Rc<RefCell<sprite::Scene<Texture>>>;


pub struct Hero {
  name: String,
  image: String,
  position: (f64, f64),
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
      position: (actor.position.x, actor.position.y),
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

  fn set_name(&mut self, name: String) -> error::Result<()> {
    self.name = name;
    Ok(())
  }

  fn set_image(&mut self, image: String) -> error::Result<()> {
    self.image = image;
    Ok(())
  }

  fn set_position(&mut self, position: (f64, f64)) -> error::Result<()> {
    self.position = position;
    Ok(())
  }

  fn set_scale(&mut self, scale: f64) -> error::Result<()> {
    self.scale = scale;
    Ok(())
  }

  fn set_visible(&mut self, visible: bool) -> error::Result<()> {
    self.visible = visible;
    Ok(())
  }

  fn set_active(&mut self, active: bool) -> error::Result<()> {
    self.active = active;
    Ok(())
  }

  fn interact_hero(&mut self) {
    println!("Hero interacted with Hero!");
  }

  fn interact_detective(&mut self) {
    println!("Hero interacted with Detective!");
  }
}


impl entity::Position for Hero {
  fn x(&self) -> f64 {
    self.position.0
  }
  fn y(&self) -> f64 {
    self.position.1
  }
  fn set_x(&mut self, new_x: f64) {
    self.position.0 = new_x;
  }
  fn set_y(&mut self, new_y: f64) {
    self.position.1 = new_y;
  }
  fn set_position(&mut self, new_x: f64, new_y: f64) {
      self.set_x(new_x);
      self.set_y(new_y);
  }
}


impl entity::Scaled for Hero {

  fn set_scale(&mut self, new_scale: f64) {
    match self.scene.borrow_mut().child_mut(self.sprite_id) {
      Some(sprite) => {
        sprite.set_scale(new_scale, new_scale);
      },
      None => { }
    }
  }

  fn get_scale(&self) -> f64 {
    match self.scene.borrow_mut().child(self.sprite_id) {
      Some(sprite) => {
        sprite.get_scale().0
      },
      None => {
        0.0
      }
    }
  }
}


impl entity::Sprited for Hero {
  fn get_sprite_id(&self) -> uuid::Uuid {
    self.sprite_id
  }
}

