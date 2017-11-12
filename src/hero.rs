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

type Texture = piston_window::G2dTexture;
type SceneRcRef = Rc<RefCell<sprite::Scene<Texture>>>;


const DEFAULT_SCALE: f64 = 1.0;


pub struct Hero {
    pos_x: f64,
    pos_y: f64,
    sprite_id: uuid::Uuid,
    scene: SceneRcRef,
}


impl Hero {
  pub fn new(assets: &assets::AssetMap, scene: SceneRcRef) -> Hero {

    let hero_texture = assets.get(&String::from("characters/detective/Detective_idle"))
        .expect("Could not find asset")
        .frames.get(0).unwrap().texture.clone();

    let mut hero_sprite = sprite::Sprite::from_texture(hero_texture);

    //hero_sprite.set_position(600.0, 775.0);
    hero_sprite.set_scale(DEFAULT_SCALE, DEFAULT_SCALE);

    let hero_id: uuid::Uuid = scene.borrow_mut().add_child(hero_sprite);

    let seq = Sequence(vec![
        While(Box::new(WaitForever), vec![
              Action(sprite::Ease(sprite::EaseFunction::ExponentialIn, Box::new(sprite::MoveBy(3.0, 0.0, 50.0)))),
              Action(sprite::Ease(sprite::EaseFunction::ExponentialIn, Box::new(sprite::MoveBy(3.0, 0.0, -50.0)))),
        ]),
        ]);
    scene.borrow_mut().run(hero_id, &seq);

    let hero = Hero {
        pos_x: 0.0,
        pos_y: 0.0,
        sprite_id: hero_id,
        scene: scene,
    };
    hero
  }
}


impl entity::Actor for Hero {
  fn interact_hero(&mut self) {
    println!("Hero interacted with Hero!");
  }

  fn interact_detective(&mut self) {
    println!("Hero interacted with Detective!");
  }
}


impl entity::Position for Hero {
  fn x(&self) -> f64 {
    self.pos_x
  }
  fn y(&self) -> f64 {
    self.pos_y
  }
  fn set_x(&mut self, new_x: f64) {
    self.pos_x = new_x;
  }
  fn set_y(&mut self, new_y: f64) {
    self.pos_y = new_y;
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

