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

type Texture = piston_window::G2dTexture;
type SceneRcRef = Rc<RefCell<sprite::Scene<Texture>>>;
pub type DetectiveRcRef = Rc<RefCell<Detective>>;

#[derive(Copy, Clone)]
enum DetectiveState {
    Idle,
    Walk,
    Clue,
}

pub struct Detective {
  name: String,
  position: entity::WorldPoint2,
  velocity: entity::WorldVector2,
  scale: f64,
  speed: f64,
  width: f64,
  visible: bool,
  active: bool,
  sprite_id: uuid::Uuid,
  scene: SceneRcRef,
  idle: Rc<assets::ImageAsset>, // asset for idle animation
  walk: Rc<assets::ImageAsset>, // asset for walk animation
  clue: Rc<assets::ImageAsset>, // asset for clue animation
  clue_sound: String,
  frame: usize,
  next_frame: f64,
  // The detective's state machine is tied to his animations; code may set
  // next_state, but he won't progress to that state until his current animation
  // is done
  state: DetectiveState,
  next_state: DetectiveState,
}


impl Detective {
  pub fn new(
    actor: &level::Detective,
    assets: &assets::AssetMap,
    scene: SceneRcRef,
  ) -> Detective {
    // Get the idle asset and add it to our internal state to asset map
    let idle = assets.get(&actor.idle)
        .expect("Could not find asset")
        .clone();
    let walk = assets.get(&actor.walk)
        .expect("Could not find asset")
        .clone();
    let clue = assets.get(&actor.clue)
        .expect("Could not find asset")
        .clone();

    // Set the current state and remaining frame time
    let frame : usize = 0;
    let frame0 = idle.frames.get(0).unwrap();
    let next_frame = frame0.frame_time;

    let hero_texture = frame0.texture.clone();

    let mut hero_sprite = sprite::Sprite::from_texture(hero_texture);

    hero_sprite.set_position(actor.position.x, actor.position.y);
    hero_sprite.set_scale(actor.scale, actor.scale);

    let hero_id: uuid::Uuid = scene.borrow_mut().add_child(hero_sprite);

    Detective {
      name: actor.name.to_owned(),
      position: entity::WorldPoint2::new(actor.position.x, actor.position.y),
      velocity: entity::WorldVector2::new(0.0, 0.0),
      scale: actor.scale,
      width: (actor.width as f64) * actor.scale,
      speed: actor.speed,
      visible: true,
      active: true,
      sprite_id: hero_id,
      scene: scene,
      idle: idle.clone(),
      walk,
      clue,
      clue_sound: actor.clue_sound.to_owned(),
      frame,
      next_frame,
      state: DetectiveState::Idle,
      next_state: DetectiveState::Idle,
    }
  }

  pub fn interact_entity(&mut self, actor: &entity::Actor) {
    // How can the detective interact with things?
    //  - barrier: detective turns around and walks the other way
    //  - clue: detective stops, inspects
    //    - if this is the macguffin, trigger level completion
    //    - if this isn't the macguffin, detective continues moving
    if actor.active() {
      //println!("Detective interacted with {}!", actor.name());
    }
  }
}


impl entity::Actor for Detective {
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
        self.next_state = DetectiveState::Walk;
        if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
          sprite.set_flip_x(self.velocity.x < 0.0);
        }
    } else {
        self.next_state = DetectiveState::Idle;
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

    // TODO(austin): make the detective do useful things here
    if self.position.x > 1000.0 {
      let speed = -self.speed;
      self.set_velocity_x(speed);
    } else if self.position.x < 200.0 {
      let speed = self.speed;
      self.set_velocity_x(speed);
    }

    // update time to next frame
    self.next_frame -= update_args.dt;

    if self.next_frame <= 0.0 {
      // if it's time for the next frame, get the asset
      let asset = match self.state {
          DetectiveState::Idle => &self.idle,
          DetectiveState::Walk => &self.walk,
          DetectiveState::Clue => &self.clue,
      };

      // get the index of the next frame
      self.frame += 1;
      if self.frame >= asset.frames.len() {
          self.frame = 0;
          // Transition to next state
          self.state = self.next_state;
          // HACK(austin): cycle states
          //self.state = match self.state {
          //    DetectiveState::Idle => DetectiveState::Walk,
          //    DetectiveState::Walk => DetectiveState::Clue,
          //    DetectiveState::Clue => DetectiveState::Idle,
          //};
      }

      // if it's time for the next frame, get the asset
      let asset = match self.state {
          DetectiveState::Idle => &self.idle,
          DetectiveState::Walk => &self.walk,
          DetectiveState::Clue => &self.clue,
      };

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

  fn interact_hero(&mut self, _sounds: &mut sound::SoundEffects) {
    println!("Hero interacted with Detective!");
    let speed = self.speed;
    self.set_velocity_x(speed);
  }

  fn interact_detective(&mut self) {
    println!("Detective interacted with Detective!");
  }
}
