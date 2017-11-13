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
  scale: f64,
  speed: f64,
  width: f64,
  text: String,
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
  last_obstacle: String,
  last_clue: String,
  direction: bool,
  found_macguffin: bool,
  done: bool,
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
      scale: actor.scale,
      width: (actor.width as f64) * actor.scale,
      text: String::from(""),
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
      last_obstacle: String::from(""),
      last_clue: String::from(""),
      direction: true,
      found_macguffin: false,
      done: false,
    }
  }

  pub fn interact_entity(&mut self, actor: &entity::Actor, sounds: &mut sound::SoundEffects) {
    use entity::Actor;
    // How can the detective interact with things?
    //  - barrier: detective turns around and walks the other way
    //  - clue: detective stops, inspects
    //    - if this is the macguffin, trigger level completion
    //    - if this isn't the macguffin, detective continues moving
    match actor.actor_type() {
      // Do nothing for static actors
      level::ActorType::Static => (),
      level::ActorType::Obstacle => {
        if self.last_obstacle != actor.name() {
          println!("Detective hit obstacle {}!", actor.name());
          self.last_obstacle = actor.name();

          // if this obstacle is "active", walk the other way
          if actor.active() {
            let dir = !self.direction;
            self.set_direction(dir);
            self.last_clue = String::from("");
          }
        }
      },
      level::ActorType::Clue(macguffin) => {
        if self.last_clue != actor.name() && actor.active() {
          self.last_clue = actor.name();
          self.next_state = DetectiveState::Clue;
          sounds.play(&self.clue_sound);

          if macguffin {
            println!("Detective found the macguffin!");
            self.found_macguffin = true;
          }
        }
      }
    }
  }

  pub fn run_away(&mut self) {
    use entity::Actor;
    self.last_obstacle = String::from("");
    let dir = !self.direction;
    self.set_direction(dir);
    self.next_state = DetectiveState::Walk;
    self.set_text(String::from("Aaaaah!!!")).expect("Failed setting text");
  }

  pub fn set_direction(&mut self, dir: bool) {
    self.direction = dir;
    if let Some(sprite) = self.scene.borrow_mut().child_mut(self.sprite_id) {
      sprite.set_flip_x(!self.direction);
    }
  }

  pub fn done(&self) -> bool {
    self.done
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
    entity::WorldVector2::new(0.0, 0.0)
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

  fn direction(&self) -> bool {
    self.direction
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
    // motion update if detective is in walking state
    match self.state {
      DetectiveState::Walk => {
        let velocity = if self.direction {
          entity::WorldVector2::new(self.speed, 0.0)
        } else {
          entity::WorldVector2::new(-self.speed, 0.0)
        };
        let new_position = self.position + velocity * update_args.dt;
        self.set_position(new_position)?;
      },
      _ => (),
    }

    // HACK(austin): keep the detective from wandering off screen
    // TODO(austin): don't use hardcoded world bounds here
    if self.position.x > 4800.0 {
      let speed = -self.speed;
      self.set_direction(false);
      self.last_obstacle = String::from("");
      self.last_clue = String::from("");
    } else if self.position.x < 150.0 {
      let speed = self.speed;
      self.set_direction(true);
      self.last_obstacle = String::from("");
      self.last_clue = String::from("");
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

          // Next-next state; this determines if a state is a one-shot or if
          // it continues
          self.next_state = match self.next_state {
              DetectiveState::Idle => {
                self.done = self.found_macguffin;
                DetectiveState::Idle
              }
              DetectiveState::Walk => DetectiveState::Walk,
              DetectiveState::Clue => {
                if self.found_macguffin {
                  DetectiveState::Idle
                } else {
                  DetectiveState::Walk
                }
              }
          };
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
    self.next_state = DetectiveState::Walk;
  }

  fn interact_detective(&mut self) {
    println!("Detective interacted with Detective!");
  }
}
