extern crate ai_behavior;
extern crate piston_window;
extern crate sprite;
extern crate uuid;
extern crate graphics;


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
use sound;

type Texture = piston_window::G2dTexture;
type SceneRcRef = Rc<RefCell<sprite::Scene<Texture>>>;
pub type HeroRcRef = Rc<RefCell<Hero>>;


const TRANSPARENT_OPACITY: f32 = 0.4;
const TRANSPARENT_SCALE_FACTOR: f64 = 0.8;

#[derive(Copy, Clone)]
enum HeroState {
  Idle,
  Ascend,
  Done,
}

pub struct Hero {
  name: String,
  position: entity::WorldPoint2,
  velocity: entity::WorldVector2,
  scale: f64,
  width: f64,
  text: String,
  visible: bool,
  active: bool,
  sprite_id: uuid::Uuid,
  scene: SceneRcRef,
  idle: Rc<assets::ImageAsset>,
  ascend: Rc<assets::ImageAsset>,
  frame: usize,
  next_frame: f64,
  transparent: bool,
  state: HeroState,
  next_state: HeroState,
}


impl Hero {
  pub fn new(
    actor: &level::Hero,
    assets: &assets::AssetMap,
    scene: SceneRcRef,
  ) -> Hero {
    // Get the idle asset and add it to our internal state to asset map
    let hero_idle = assets.get(&actor.idle)
        .expect("Could not find asset")
        .clone();
    let hero_ascend = assets.get(&actor.ascend)
        .expect("Could not find asset")
        .clone();

    // Set the remaining frame time
    let frame : usize = 0;
    let frame0 = hero_idle.frames.get(0).unwrap();
    let next_frame = frame0.frame_time;

    let hero_texture = frame0.texture.clone();

    let mut hero_sprite = sprite::Sprite::from_texture(hero_texture);

    hero_sprite.set_position(actor.position.x, actor.position.y);
    hero_sprite.set_scale(actor.scale * TRANSPARENT_SCALE_FACTOR,
                          actor.scale * TRANSPARENT_SCALE_FACTOR);
    hero_sprite.set_opacity(TRANSPARENT_OPACITY);

    let hero_id: uuid::Uuid = scene.borrow_mut().add_child(hero_sprite);

    Hero {
      name: actor.name.to_owned(),
      position: entity::WorldPoint2::new(actor.position.x, actor.position.y),
      velocity: entity::WorldVector2::new(0.0, 0.0),
      scale: actor.scale,
      width: (actor.width as f64) * actor.scale,
      text: String::from(""),
      visible: true,
      active: true,
      sprite_id: hero_id,
      scene: scene,
      idle: hero_idle.clone(),
      ascend: hero_ascend.clone(),
      frame,
      next_frame,
      transparent: true,
      state: HeroState::Idle,
      next_state: HeroState::Idle,
    }
  }

  pub fn is_transparent(&self) -> bool {
    self.transparent
  }

  pub fn turn_transparent(&mut self) -> error::Result<()> {
    self.transparent = true;
    if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
      sprite.set_opacity(TRANSPARENT_OPACITY);
      let current_scale = sprite.get_scale();
      sprite.set_scale(current_scale.0 * TRANSPARENT_SCALE_FACTOR,
                       current_scale.1 * TRANSPARENT_SCALE_FACTOR);
    }
    Ok(())
  }

  pub fn turn_opaque(&mut self) -> error::Result<()> {
    self.transparent = false;
    if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
      sprite.set_opacity(1.0);
      sprite.set_scale(self.scale, self.scale);
    }
    Ok(())
  }

  pub fn ascend(&mut self) {
    self.next_state = match self.next_state {
        HeroState::Idle => HeroState::Ascend,
        HeroState::Ascend => HeroState::Ascend,
        HeroState::Done => HeroState::Done,
    }
  }

  pub fn won(&self) -> bool {
    match self.state {
      HeroState::Done => true,
      _ => false,
    }
  }
}


impl entity::Actor for Hero {
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

  fn width(&self) -> f64 {
    self.width
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
    if self.velocity.x != 0.0 {
        if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
          sprite.set_flip_x(self.velocity.x < 0.0);
        }
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

  fn text(&self) -> &String {
    &self.text
  }
  fn set_text(&mut self, new_text: String) -> error::Result<()> {
    self.text = new_text;
    Ok(())
  }

  fn on_update(&mut self, update_args: &piston_window::UpdateArgs) -> error::Result<()> {
    let new_position = self.position + self.velocity * update_args.dt;
    self.set_position(new_position)?;

    // update time to next frame
    self.next_frame -= update_args.dt;

    if self.next_frame <= 0.0 {
      // if it's time for the next frame, get the asset
      let asset = match self.state {
        HeroState::Idle => &self.idle,
        HeroState::Ascend => &self.ascend,
        HeroState::Done => &self.ascend,
      };

      // get the index of the next frame
      self.frame += 1;
      if self.frame >= asset.frames.len() {
          // TODO(austin): this is where we could update state to chain in the
          // next animation
          self.frame = 0;
          self.state = self.next_state;

          self.next_state = match self.next_state {
            HeroState::Idle => HeroState::Idle,
            HeroState::Ascend => HeroState::Done,
            HeroState::Done => HeroState::Done,
          };
      }

      let asset = match self.state {
        HeroState::Idle => &self.idle,
        HeroState::Ascend => &self.ascend,
        HeroState::Done => &self.ascend,
      };

      // Get the next frame
      self.frame = match self.state{
        HeroState::Done => asset.frames.len()-1,
        _ => self.frame,
      };
      let frame = asset.frames.get(self.frame).unwrap();

      // Set the frame time and the update the sprite
      self.next_frame += frame.frame_time;
      if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
          sprite.set_texture(frame.texture.clone());
      }
    }

    Ok(())
  }

  fn interact_hero(&mut self, _sounds: &mut sound::SoundEffects) {
    println!("Hero interacted with Hero!");
  }

  fn interact_detective(&mut self) {
    println!("Hero interacted with Detective!");
  }
}
