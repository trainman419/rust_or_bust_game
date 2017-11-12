#[cfg(unix)]
extern crate ears;
extern crate graphics;
extern crate nalgebra;
extern crate piston;
extern crate piston_window;
extern crate find_folder;
extern crate sprite;
extern crate tiled;
extern crate gif;
extern crate image;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::thread;
use std::time;

#[cfg(unix)]
use self::ears::{Sound, Music, AudioController};

use std::fs::File;
use std::path::Path;

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
    let volume = 1.0;

    match file {
      "cans" => {
        filename = "cans.wav";
        max_length = 1500;
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
      println!("Could not find file");
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
          self.sound_effects.play("test");
        },
        // TODO: these should come from config.
        piston_window::Key::Left => {
          self.state.camera.velocity.x = -500.0;
        },
        piston_window::Key::Right => {
          self.state.camera.velocity.x = 500.0;
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
        piston_window::Key::Left => {
          self.state.camera.velocity.x = 0.0;
        },
        piston_window::Key::Right => {
          self.state.camera.velocity.x = 0.0;
        },
        _ => {},
      },
      _ => {},
    }

    Ok(())
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
    for (ref _name, ref entity) in self.state.entities.iter() {
      entity.borrow_mut().on_update(update_args);
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
    use self::graphics::Transformed; // piston_window::Context.{trans,orient}

    self.window.borrow_mut().draw_2d(event, |context, graphics| {
      let translation = self.state.camera.position;
      let transform = context
        .trans(-translation.x, translation.y)
        .zoom(self.state.camera.zoom)
        .transform;

      piston_window::clear([1.0; 4], graphics);
      self.scene.borrow_mut().draw(transform, graphics);
    });

    Ok(())
  }
}

fn load_assets_from_dir<Window>(
  mut window: &mut piston_window::PistonWindow<Window>,
  dir: &Path,
  prefix: &str,
  mut assets: &mut assets::AssetMap)
where Window: piston_window::Window
{
  for entry in dir.read_dir().expect("read dir call failed") {
    if let Ok(entry) = entry {
      if entry.file_type().unwrap().is_dir() {

        let name = if prefix.len() > 0 {
            prefix.to_owned() + "/" + entry.file_name().to_str().unwrap()
        } else {
            entry.file_name().to_str().unwrap().to_owned()
        };

        load_assets_from_dir(&mut window, &entry.path(), &name, &mut assets);
      } else if entry.file_type().unwrap().is_file() {
        let path = entry.path();
        let name = if prefix.len() > 0 {
            prefix.to_owned() + "/" + path.file_stem().unwrap().to_str().unwrap()
        } else {
            path.file_stem().unwrap().to_str().unwrap().to_owned()
        };
        if let Some(extension) = entry.path().extension() {
          match extension.to_str().unwrap() {
              "png" => {
                  println!("Loading {}", name);
                  let texture = Rc::new(piston_window::Texture::from_path(
                                        &mut window.factory,
                                        path,
                                        piston_window::Flip::None,
                                        &piston_window::TextureSettings::new().mag(piston_window::Filter::Nearest),
                                        ).unwrap());
                  let mut asset = assets::ImageAsset::new();
                  asset.add_frame(texture, 0.0);
                  assets.insert(name, Rc::new(asset));
              }
              "gif" => {
                  use self::gif::Decoder;
                  use self::gif::SetParameter;
                  println!("Loading {}", name);
                  let mut asset = assets::ImageAsset::new();

                  let mut decoder = Decoder::new(File::open(&path).expect(&format!("Could not open {:?}", &path)));
                  decoder.set(gif::ColorOutput::RGBA);
                  let mut decoder = decoder.read_info().expect(&format!("Could not decode gif {:?}", &path));

                  let size = (decoder.width() as u32, decoder.height() as u32);
                  let frame_size = (size.0 * size.1 * 4) as usize;
                  while let Some(frame) = decoder.read_next_frame().expect(&format!("Could not read next frame from {:?}", &path)) {
                      use self::image::GenericImage;
                      let cur_frame = vec![0u8; frame_size];
                      let src = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(frame.width as u32, frame.height as u32, frame.buffer.clone().into_owned()).expect("Could not create source image (source too small?)");
                      let mut dst = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(size.0, size.1, cur_frame).expect("Could not create destination image buffer");
                      dst.copy_from(&src, frame.left as u32, frame.top as u32);

                      let texture = Rc::new(piston_window::Texture::from_image(
                                            &mut window.factory,
                                            &dst,
                                            &piston_window::TextureSettings::new().mag(piston_window::Filter::Nearest),
                                            ).expect("Could not create Texture"));
                      // convert frame time from 10ms units to floating-point seconds
                      asset.add_frame(texture, (frame.delay as f64) / 100.0);
                  }
                  assets.insert(name, Rc::new(asset));
              }
              _ => (),
          }
        }
      }
    }
  }
}

fn load_assets<Window>(mut window: &mut piston_window::PistonWindow<Window>) -> assets::AssetMap
where Window: piston_window::Window
{
  let mut assets = HashMap::new();
  // Load assets. This probably isn't the place, but we'll deal with that
  // later.

  let asset_dir = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();

  load_assets_from_dir(&mut window, &asset_dir, "", &mut assets);

  return assets;
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
    let mut camera = camera::Camera2::new();
    camera.position.y = 800.0;

    let assets = load_assets(&mut window.borrow_mut());

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
