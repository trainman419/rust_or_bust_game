extern crate ai_behavior;
extern crate piston_window;
extern crate sprite;
extern crate uuid;

//use self::ai_behavior::{
//    Action,
//    Sequence,
//    WaitForever,
//    While,
//};

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
  assets: assets::AssetMap, // assets for each state or animation of the hero
  state: String,
  frame: usize,
  next_frame: f64,
}


impl Hero {
  pub fn new(
    actor: &level::Actor,
    assets: &assets::AssetMap,
    scene: SceneRcRef,
  ) -> Hero {
    let mut hero_assets = assets::AssetMap::new();

    // Get the idle asset and add it to our internal state to asset map
    let hero_idle = assets.get(&actor.image)
        .expect("Could not find asset")
        .clone();
    hero_assets.insert(String::from("idle"), hero_idle.clone());

    // Set the current state and remaining frame time
    let state = String::from("idle");
    let frame : usize = 0;
    let frame0 = hero_idle.frames.get(0).unwrap();
    let next_frame = frame0.frame_time;

    let hero_texture = frame0.texture.clone();

    let mut hero_sprite = sprite::Sprite::from_texture(hero_texture);

    hero_sprite.set_position(actor.position.x, actor.position.y);
    hero_sprite.set_scale(actor.scale, actor.scale);

    let hero_id: uuid::Uuid = scene.borrow_mut().add_child(hero_sprite);

    //let seq = Sequence(vec![
    //    While(Box::new(WaitForever), vec![
    //          Action(sprite::Ease(sprite::EaseFunction::ExponentialIn, Box::new(sprite::MoveBy(3.0, 0.0, 50.0)))),
    //          Action(sprite::Ease(sprite::EaseFunction::ExponentialIn, Box::new(sprite::MoveBy(3.0, 0.0, -50.0)))),
    //    ]),
    //    ]);
    //scene.borrow_mut().run(hero_id, &seq);

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
      assets: hero_assets,
      state,
      frame,
      next_frame,
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
    if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
      sprite.set_flip_x(self.velocity.x < 0.0);
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

  fn on_update(&mut self, update_args: &piston_window::UpdateArgs) -> error::Result<()> {
    let new_position = self.position + self.velocity * update_args.dt;
    self.set_position(new_position)?;

    // update time to next frame
    self.next_frame -= update_args.dt;

    if self.next_frame <= 0.0 {
      // if it's time for the next frame, get the asset
      let asset = self.assets.get(&self.state).unwrap();

      // get the index of the next frame
      self.frame += 1;
      if self.frame >= asset.frames.len() {
          // TODO(austin): this is where we could update state to chain in the
          // next animation
          self.frame = 0;
      }

      // Get the next frame
      let frame = asset.frames.get(self.frame).unwrap();

      // Set the frame time and the update the sprite
      self.next_frame += frame.frame_time;
      if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
          sprite.set_texture(frame.texture.clone());
      }
    }

    Ok(())
  }

  fn interact_hero(&mut self) {
    println!("Hero interacted with Hero!");
  }

  fn interact_detective(&mut self) {
    println!("Hero interacted with Detective!");
  }
}
