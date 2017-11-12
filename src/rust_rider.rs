#[cfg(unix)]
extern crate ears;
extern crate graphics;
extern crate nalgebra;
extern crate piston;
extern crate piston_window;
extern crate sprite;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::thread;
use std::time;

#[cfg(unix)]
use self::ears::{Sound, Music, AudioController};

use assets;
use camera;
use default_actor;
use entity;
use error;
use handler;
use hero;
use level;

type Texture = piston_window::G2dTexture;
type SceneRcRef = Rc<RefCell<sprite::Scene<Texture>>>;
type Scene = sprite::Scene<Texture>;

pub struct SoundEffects {
  music: Option<thread::JoinHandle<()>>,
  sounds: Vec<thread::JoinHandle<()>>,
}

impl SoundEffects {
  pub fn new() -> SoundEffects {
    SoundEffects {
      music: None,
      sounds: Vec::new(),
    }
  }

  #[cfg(not(unix))]
  pub fn start_music(&mut self) {
    println!("Error: Cannot play sound on Windows");
  }

  #[cfg(not(unix))]
  pub fn play(&mut self, file: &str) {
    println!("Error: Cannot play sound on Windows");
  }

  #[cfg(unix)]
  pub fn start_music(&mut self) {
    if self.music.is_none() {
        let path = String::from("assets/sounds/music/strangeness.ogg");
        let handle = thread::spawn(move || {
          let mut music = Music::new(&path).unwrap();
          music.set_looping(true);
          music.set_volume(0.25);
          music.play();
          while music.is_playing() {
            thread::sleep(time::Duration::from_secs(1));
            // Todo: Maybe something here
          }
        });
        self.music = Some(handle);
    } else {
        println!("Stop right there criminal scum.");
    }
  }


  #[cfg(unix)]
  pub fn play(&mut self, file: &str) {
    let mut path = String::from("assets/sounds/effects/");
    let mut filename = "";
    let mut max_length = 0;
    let mut volume = 1.0;

    match file {
      "cans" => {
        filename = "cans.wav";
        max_length = 1500;
      },
      "car_horn" => {
          filename = "car_horn.wav";
          volume = 0.25;
      },
      "crow_squawk" => filename = "crow_squawk.wav",
      "crunchy_leaf" => filename = "crunchy_leaf.wav",
      "foliage_rustle" => filename = "foliage_rustle.wav",
      "rocks" => filename = "rocks.wav",
      "spooked_birds" => filename = "spooked_birds.wav",
      "twig_snap" => filename = "twig_snap.wav",
      _ => {}
    }

    if filename == "" {
      println!("Could not find file: {}", file);
      return ();
    }

    path.push_str(filename);
    let handle = thread::spawn(move || {
      let mut sound = Sound::new(&path).unwrap();
        sound.set_volume(volume);
        sound.play();
        if max_length > 0 {
            thread::sleep(time::Duration::from_millis(max_length));
        } else {
           while sound.is_playing() { }
        }
    });

    self.sounds.push(handle);
  }
}

/// The game-ion of the Rust Rider game. The state should act as the save data
/// for a resumable session of the game.
pub struct State {
  level: level::Level,
  camera: camera::Camera2,
  entities: entity::EntityMap,
}

impl State {
  /// Create a State with default values for a new game.
  pub fn new(level: level::Level, camera: camera::Camera2) -> State {
    State {
      level: level,
      camera: camera,
      entities: entity::EntityMap::new(),
    }
  }
}

pub struct GameMode<Window>
where
  Window: piston_window::Window,
{
  state: State,
  window: Rc<RefCell<piston_window::PistonWindow<Window>>>,
  assets: assets::AssetMap,
  scene: SceneRcRef,
  sound_effects: SoundEffects,
}

/// How GameMode responds to input-events.
impl<Window> handler::InputHandler for GameMode<Window>
where Window: piston_window::Window,
{
  fn on_press<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    button: &piston_window::Button,
  ) -> error::Result<()> {
    match button {
      &piston_window::Button::Keyboard(key) => match key {
        piston_window::Key::X => {
          self.sound_effects.play("car_horn");
        },
        // TODO: these speeds should come from config.
        piston_window::Key::Left => {
          if let Some(hero) = self.state.entities.get("hero") {
            let mut velocity = hero.borrow().velocity();
            velocity.x = -500.0;
            hero.borrow_mut().set_velocity(velocity)?;
          }
        },
        piston_window::Key::Right => {
          if let Some(hero) = self.state.entities.get("hero") {
            let mut velocity = hero.borrow().velocity();
            velocity.x = 500.0;
            hero.borrow_mut().set_velocity(velocity)?;
          }
        },
        _ => {},
      },
      _ => {},
    }

    Ok(())
  }

  fn on_release<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    button: &piston_window::Button,
  ) -> error::Result<()> {
    match button {
      &piston_window::Button::Keyboard(key) => match key {
        piston_window::Key::Left | piston_window::Key::Right => {
          if let Some(hero) = self.state.entities.get("hero") {
            let mut velocity = hero.borrow().velocity();
            velocity.x = 0.0;
            hero.borrow_mut().set_velocity(velocity)?;
          }
        },
        _ => {},
      },
      _ => {},
    }

    Ok(())
  }
}

fn clamp<T: ::std::cmp::PartialOrd>(x: T, min: T, max: T) -> T {
  if x < min {
    min
  } else if x > max {
    max
  } else {
    x
  }
}

/// How GameMode responds to update-events.
impl<Window> handler::UpdateHandler for GameMode<Window>
where Window: piston_window::Window,
{
  fn on_update<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    update_args: &piston_window::UpdateArgs,
  ) -> error::Result<()> {
    use piston_window::Window; // size

    for (ref _name, ref entity) in self.state.entities.iter() {
      entity.borrow_mut().on_update(update_args)?;
    }

    if let Some(hero) = self.state.entities.get("hero") {
      let mut hero_position = hero.borrow().position();
      // TODO: find a better solution than padding here.
      // Intersection with bounds should account for size for actor.
      // Camera size is based on size of window.
      // Other size might be based on size of sprite or collision box.
      hero_position.x = clamp(
        hero_position.x,
        self.state.level.world_bounds.0.x + 75.0,
        self.state.level.world_bounds.1.x - 75.0,
      );
      hero.borrow_mut().set_position(hero_position)?;

      let window_size = self.window.borrow().size();
      self.state.camera.position.x = hero_position.x;
      self.state.camera.position.x = clamp(
        self.state.camera.position.x,
        self.state.level.world_bounds.0.x + window_size.width as f64 * 0.5,
        self.state.level.world_bounds.1.x - window_size.width as f64 * 0.5,
      );
    }

    Ok(())
  }
}

/// How GameMode responds to window-events.
impl<Window> handler::WindowHandler for GameMode<Window>
where Window: piston_window::OpenGLWindow,
{
  fn on_render<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    _render_args: &piston_window::RenderArgs,
  ) -> error::Result<()> {
    use piston_window::Window; // size
    use self::graphics::Transformed; // piston_window::Context.{trans,orient}

    // Borrow member references immutably before allowing self to be borrowed
    // mutably by self.window.draw_2d().
    let window_size = self.window.borrow().size();

    self.window.borrow_mut().draw_2d(event, |context, graphics| {
      let translation = self.state.camera.position;
      let transform = context
        .trans(
          window_size.width as f64 * 0.5 - translation.x,
          window_size.height as f64 * 0.5 + translation.y,
        )
        .zoom(self.state.camera.zoom)
        .transform;

      piston_window::clear([1.0; 4], graphics);
      self.scene.borrow_mut().draw(transform, graphics);
    });

    Ok(())
  }
}

/// Inherit default implementation of EventHandler::on_event.
impl<Window> handler::EventHandler for GameMode<Window>
where Window: piston_window::OpenGLWindow,
{
  fn before_event<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
  ) -> error::Result<()> {
    self.scene.borrow_mut().event(event);
    Ok(())
  }
}

fn make_actor(
  actor: &level::Actor,
  assets: &assets::AssetMap,
  scene: SceneRcRef,
) -> Rc<RefCell<entity::Actor>> {
  if actor.name == "hero" {
    Rc::new(RefCell::new(hero::Hero::new(actor, assets, scene.clone())))
  } else if actor.name == "detective" {
    Rc::new(RefCell::new(hero::Hero::new(actor, assets, scene.clone())))
  } else {
    Rc::new(RefCell::new(default_actor::DefaultActor::new(actor, assets, scene.clone())))
  }
}

impl<Window> GameMode<Window>
where
  Window: piston_window::Window + piston_window::OpenGLWindow,
{
  /// Create a GameMode for a new game.
  pub fn new(
    window: Rc<RefCell<piston_window::PistonWindow<Window>>>,
  ) -> GameMode<Window> {
    // TODO: should be loaded as an actor from level
    let camera = camera::Camera2::new();

    let assets = assets::load_assets(&mut window.borrow_mut());

    let level = level::Level::from_path_str("assets/levels/sample.json")
        .expect("Failed to load level");
    let scene = Rc::new(RefCell::new(Scene::new()));
    let mut state = State::new(level.clone(), camera);

    for actor in level.actors.iter() {
      state.entities.insert(
        actor.name.to_owned(),
        make_actor(&actor, &assets, scene.clone()),
      );
    }

    let mut sound_effects = SoundEffects::new();
    sound_effects.start_music();

    GameMode::new_with_state(window, state, assets, scene.clone(), sound_effects)
  }

  /// Create a GameMode with an existing State.
  pub fn new_with_state(
    window: Rc<RefCell<piston_window::PistonWindow<Window>>>,
    state: State,
    assets: assets::AssetMap,
    scene: SceneRcRef,
    sound_effects: SoundEffects,
  ) -> GameMode<Window> {
    GameMode {
      window,
      state,
      assets,
      scene,
      sound_effects,
    }
  }
}
