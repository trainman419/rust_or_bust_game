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

use self::ears::{Sound, Music, AudioController};

use std::fs::File;
use std::path::Path;

use assets;
use camera;
use entity;
use entity::Position;
use entity::Scaled;
use entity::Sprited;
use error;
use handler;
use hero;

enum EditMode {
  Insert,
  Select,
}

type Point = nalgebra::Point2<f64>;
type Vector = nalgebra::Vector2<f64>;

type Texture = piston_window::G2dTexture;


type SceneRcRef = Rc<RefCell<sprite::Scene<Texture>>>;

type Scene = sprite::Scene<Texture>;

fn draw_rectangle<G>(
  point1: &Point,
  point2: &Point,
  context: &piston_window::Context,
  graphics: &mut G,
) where
  G: graphics::Graphics,
{
  use self::graphics::Transformed; // piston_window::Context.{trans,orient}
  let delta = point2 - point1;
  piston_window::rectangle(
    CYAN,
    [0.0, 0.0, delta[0], delta[1]],
    context
      .trans(point1.x, point1.y)
      .transform,
    graphics,
  );
}

fn draw_line_segment<G>(
  point1: &Point,
  point2: &Point,
  context: &piston_window::Context,
  graphics: &mut G,
) where
  G: graphics::Graphics,
{
  use self::graphics::Transformed; // piston_window::Context.{trans,orient}

  let tangent = point2 - point1;
  let width = nalgebra::distance(point1, point2);
  let height = 4.0;
  piston_window::rectangle(
    BLACK,
    [0.0, 0.0, width, height],
    context
      .trans(point1.x, point1.y - height / 2.0)
      .orient(tangent.x, tangent.y)
      .transform,
    graphics,
  );
}

struct LineSegment {
  point1: Point,
  point2: Point,
}

impl LineSegment {
  pub fn new(point1: Point, point2: Point) -> LineSegment {
    LineSegment {
      point1: point1,
      point2: point2,
    }
  }

  pub fn draw<G>(&self, context: &piston_window::Context, graphics: &mut G)
  where
    G: graphics::Graphics,
  {
    draw_line_segment(&self.point1, &self.point2, context, graphics);
  }
}

const CYAN:  piston_window::types::Color = [0.0, 1.0, 1.0, 0.5];
const BLACK: piston_window::types::Color = [0.0, 0.0, 0.0, 1.0];
const GREEN: piston_window::types::Color = [0.0, 1.0, 0.0, 1.0];
const BLUE:  piston_window::types::Color = [0.0, 0.0, 1.0, 1.0];

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

  pub fn start_music(&mut self) {
    if self.music.is_none() {
        let path = String::from("assets/sounds/music/strangeness.ogg");
        let handle = thread::spawn(move || {
          let mut music = Music::new(&path).unwrap();
          music.set_looping(true);
          music.set_volume(0.25);
          music.play();
          while music.is_playing() {
            // Todo: Maybe something here
          }
        });
        self.music = Some(handle);
    } else {
        println!("Stop right there criminal scum.");
    }
  }

  pub fn play(&mut self, file: &str) {
    let mut path = String::from("assets/sounds/effects/");
    let mut filename = "";

    match file {
      "test" => filename = "bullet-shell.wav",
      _ => {}
    }

    if filename == "" {
      println!("Could not find file");
      return ();
    }

    path.push_str(filename);
    let handle = thread::spawn(move || {
      let mut sound = Sound::new(&path).unwrap();
      sound.play();
      while sound.is_playing() {
        // Todo: Maybe something here
      }
    });

    self.sounds.push(handle);
  }
}

/// The game-ion of the Rust Rider game. The state should act as the save data
/// for a resumable session of the game.
pub struct State {
  edit_mode: EditMode,
  line_segments: Vec<LineSegment>,
  active_line_segment: Option<Point>,
  active_selection: Option<Point>,
  mouse_position: Point,
  camera: camera::Camera2,
  entities: entity::EntityMap,
}

impl State {
  /// Create a State with default values for a new game.
  pub fn new(camera: camera::Camera2) -> State {
    State {
      edit_mode: EditMode::Insert,
      line_segments: Vec::new(),
      active_line_segment: None,
      active_selection: None,
      mouse_position: Point::new(0.0, 0.0),
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
  fn on_mouse_cursor<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    position: &[f64; 2],
  ) -> error::Result<()> {
    self.state.mouse_position = Point::new(position[0], position[1]);

    Ok(())
  }

  fn on_press<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    button: &piston_window::Button,
  ) -> error::Result<()> {
    match button {
      &piston_window::Button::Keyboard(key) => match key {
        // This is a dirty way to just close the game.
        piston_window::Key::Q => {
          return Err(error::Error::from("Exited Game"));
        },
        piston_window::Key::X => {
          self.sound_effects.play("test");
        },
        piston_window::Key::LShift | piston_window::Key::RShift => {
          self.state.edit_mode = EditMode::Select;
        },
        piston_window::Key::Left => {
          self.state.camera.velocity.x = -500.0;
        },
        piston_window::Key::Right => {
          self.state.camera.velocity.x = 500.0;
        },
        piston_window::Key::Up => {
          self.state.camera.velocity.y = 500.0;
        },
        piston_window::Key::Down => {
          self.state.camera.velocity.y = -500.0;
        },
        _ => {},
      },
      &piston_window::Button::Mouse(mouse_button) => match mouse_button {
        piston_window::MouseButton::Left => {
          match self.state.edit_mode {
            EditMode::Select => {
              self.state.active_selection = Some(self.state.mouse_position);
            },
            EditMode::Insert => {
              self.state.active_line_segment = Some(self.state.mouse_position);
            },
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
        piston_window::Key::LShift | piston_window::Key::RShift => {
          self.state.edit_mode = EditMode::Insert;
        },
        piston_window::Key::Left => {
          self.state.camera.velocity.x = 0.0;
        },
        piston_window::Key::Right => {
          self.state.camera.velocity.x = 0.0;
        },
        piston_window::Key::Up => {
          self.state.camera.velocity.y = 0.0;
        },
        piston_window::Key::Down => {
          self.state.camera.velocity.y = 0.0;
        },
        _ => {},
      },
      &piston_window::Button::Mouse(mouse_button) => match mouse_button {
        piston_window::MouseButton::Left => {
          match self.state.active_selection {
            Some(_) => {
              self.state.active_selection = None;
            }
            None => {}
          }
          match self.state.active_line_segment {
            Some(point1) => {
              self.state.line_segments.push(
                LineSegment::new(point1, self.state.mouse_position));
              self.state.active_line_segment = None;
            }
            None => {}
          }
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
    self.state.camera.on_update(update_args);

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
    let state = &self.state;
    let window_size = self.window.borrow().size();

    self.window.borrow_mut().draw_2d(event, |context, graphics| {
      let edit_bar_color = match state.edit_mode {
        EditMode::Insert => GREEN,
        EditMode::Select => BLUE,
      };
      let edit_bar_width = window_size.width;
      let edit_bar_height = 20;
      let edit_bar_x_offset = 0;
      let edit_bar_y_offset = window_size.height - edit_bar_height;

      piston_window::clear([1.0; 4], graphics);
      piston_window::rectangle(
        edit_bar_color,
        [
          edit_bar_x_offset as f64,
          edit_bar_y_offset as f64,
          edit_bar_width as f64,
          edit_bar_height as f64,
        ],
        context.transform,
        graphics,
      );

      match state.active_selection {
        Some(point1) => {
          draw_rectangle(&point1, &state.mouse_position, &context, graphics);
        }
        None => {}
      }

      match state.active_line_segment {
        Some(point1) => {
          draw_line_segment(&point1, &state.mouse_position, &context, graphics);
        }
        None => {}
      }

      for line in state.line_segments.iter() {
        line.draw(&context, graphics);
      }

      self.scene.borrow_mut().draw(
        context.trans(-self.state.camera.position.x,
                      self.state.camera.position.y).transform,
        graphics,
      );
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
        let name = prefix.to_owned() + "/" + path.file_stem().unwrap().to_str().unwrap();
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
                  asset.add_frame(texture, 0);
                  assets.insert(name, asset);
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
                      asset.add_frame(texture, frame.delay);
                  }
                  assets.insert(name, asset);
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

impl<Window> GameMode<Window>
where
  Window: piston_window::Window + piston_window::OpenGLWindow,
{
  /// Create a GameMode for a new game.
  pub fn new(
    window: Rc<RefCell<piston_window::PistonWindow<Window>>>,
  ) -> GameMode<Window> {
    use piston_window::Window; // size
    let window_size = window.borrow().size();
    let viewport = camera::WorldVector2::new(window_size.width as f64,
                                             window_size.height as f64);

    let camera = camera::Camera2 {
      viewport: viewport,
      position: camera::WorldPoint2::new(0.0, 0.0),
      velocity: camera::WorldVector2::new(0.0, 0.0),
    };
    let assets = load_assets(&mut window.borrow_mut());
    let mut scene = Rc::new(RefCell::new(Scene::new()));
    let mut state = State::new(camera);

    // Build the default hero
    let hero_scale = 10.0;
    let mut hero = hero::Hero::new(&assets, scene.clone());
    hero.set_position(600.0, 775.0);
    hero.set_scale(hero_scale);

    let mut sound_effects = SoundEffects::new();
    sound_effects.start_music();

    let hero_id = hero.get_sprite_id();
    state.entities.insert(String::from("hero"), Rc::new(hero));

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
      window: window,
      state: state,
      assets,
      scene,
      sound_effects,
    }
  }
}
