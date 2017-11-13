extern crate find_folder;
extern crate graphics;
extern crate nalgebra;
extern crate piston;
extern crate piston_window;
extern crate sprite;

use std::cell::RefCell;
use std::rc::Rc;

use assets;
use camera;
use default_actor;
use entity;
use entity::Actor;
use error;
use font;
use handler;
use hero;
use detective;
use level;
use sound;

type Texture = piston_window::G2dTexture;
type SceneRcRef = Rc<RefCell<sprite::Scene<Texture>>>;
type Scene = sprite::Scene<Texture>;

/// The game-ion of the Rust Rider game. The state should act as the save data
/// for a resumable session of the game.
pub struct State {
  level: level::Level,
  camera: camera::Camera2,
  entities: entity::EntityMap,
  hero: Option<hero::HeroRcRef>,
  detective: Option<detective::DetectiveRcRef>,
  found: bool,
  win: bool,
  title_text: font::FontTransition,
  hint_text: font::FontTransition,
}

impl State {
  /// Create a State with default values for a new game.
  pub fn new(level: level::Level, camera: camera::Camera2) -> State {
    State {
      level: level,
      camera: camera,
      entities: entity::EntityMap::new(),
      hero: None,
      detective: None,
      win: false,
      found: false,
      title_text: font::FontTransition::new(vec![
                                              String::from("It was a dark and stormy night..."),
                                              String::from("And you've just been murdered in cold blood."),
                                              String::from("Go find help."),
                                            ],
                                            10),
      hint_text: font::FontTransition::new(vec![
                                             String::from("Use the arrow keys to haunt around"),
                                             String::from("LShift to materialize, Space to interact"),
                                           ],
                                           15),
    }
  }

  pub fn get_hero(&self) -> hero::HeroRcRef {
    let hero_opt = self.hero.clone();
    hero_opt.unwrap()
  }

  pub fn get_detective(&self) -> detective::DetectiveRcRef {
    let detective_opt = self.detective.clone();
    detective_opt.unwrap()
  }
}

pub struct GameMode<Window>
where
  Window: piston_window::Window,
{
  state: State,
  window: Rc<RefCell<piston_window::PistonWindow<Window>>>,
  //assets: assets::AssetMap,
  scene: SceneRcRef,
  sound_effects: sound::SoundEffects,
  glyphs: Rc<RefCell<piston_window::Glyphs>>,
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
          self.sound_effects.play("clue");
        },
        // TODO: these speeds should come from config.
        piston_window::Key::Left => {
          let mut hero = self.state.get_hero();
          let mut velocity = hero.borrow().velocity();
          velocity.x = -500.0;
          hero.borrow_mut().set_velocity(velocity)?;
        },
        piston_window::Key::Right => {
          let mut hero = self.state.get_hero();
          let mut velocity = hero.borrow().velocity();
          velocity.x = 500.0;
          hero.borrow_mut().set_velocity(velocity)?;
        },
        piston_window::Key::LShift => {
          let mut hero = self.state.get_hero();
          hero.borrow_mut().set_text(String::from("Boo!"), 1.0)?;
          hero.borrow_mut().turn_opaque()?;
        },
        piston_window::Key::Space => {
          let mut hero = self.state.get_hero();
          for (ref _name, ref entity) in self.state.entities.iter() {
            if !hero.borrow().is_transparent() && entity.borrow().overlap(&*hero.borrow()) {
              println!("Hero interacting with {}", entity.borrow().name());
              entity.borrow_mut().interact_hero(&mut self.sound_effects);
            }
          }
        }
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
          let mut hero = self.state.get_hero();
          let mut velocity = hero.borrow().velocity();
          velocity.x = 0.0;
          hero.borrow_mut().set_velocity(velocity)?;
        },
        piston_window::Key::LShift => {
          let mut hero = self.state.get_hero();
          hero.borrow_mut().turn_transparent()?;
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

    let hero = self.state.get_hero();
    let detective = self.state.get_detective();

    // Call on_update on entities, to move them and update their animations
    for (ref _name, ref entity) in self.state.entities.iter() {
      entity.borrow_mut().on_update(update_args)?;
    }

    // Give the detective a chance to interact with other active objects in the
    // scene
    for (ref _name, ref entity) in self.state.entities.iter() {
      let entity = entity.borrow();
      if entity.name() != "detective" {
        if entity.overlap(&*detective.borrow()) {
          detective.borrow_mut().interact_entity(&*entity, &mut self.sound_effects);
        }
      }
    }

    if detective.borrow().done() && !self.state.found {
      hero.borrow_mut().ascend();
      self.state.found = true;
      self.state.title_text = font::FontTransition::new(vec![
          String::from("The detective found your body!"),
          String::from("You may finally move on to the afterlife"),
          String::from("You win! ... ?"),
        ],
        4);
    }

    if hero.borrow().won() && !self.state.win {
      println!("The detective found your body! You win!");
      self.state.win = true;
    }

    // If the detective sees the hero, make him turn around and go the other
    // way. Maybe add some text and screaming?
    {
      let hero_position = hero.borrow().position();
      let det_position = detective.borrow().position();
      let det_direction = detective.borrow().direction();
      let dx = (hero_position.x - det_position.x).abs();
      // If distance between detective and ghost is too close, and the ghost
      // is visible
      if dx < 800.0 && !hero.borrow().is_transparent() {
        // and detective is facing the ghost...
        if (hero_position.x > det_position.x) == det_direction {
          println!("Detective sees the ghost!");
          detective.borrow_mut().run_away();
          // TODO(austin): startled or scream noise
        }
      }
    }

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

      // Draw text labels over all actors with text
      for (ref _name, ref entity) in self.state.entities.iter() {
        let entity = entity.borrow();
        if entity.text().len() > 0 {
          // Add some magic numbers to make the text line up in the right spot
          // TODO(daniel): Figure out how to center the text
          let label_tf = transform.trans(entity.position().x - 35.0,
                                         entity.position().y - 125.0);
          piston_window::text::Text::new_color([1.0, 1.0, 1.0, 1.0], 4).draw(
              entity.text(),
              &mut *self.glyphs.borrow_mut(),
              &context.draw_state,
              label_tf,
              graphics
          ).expect("Failed drawing label");
        }
      }

      let transform = context.transform.trans(50.0, 100.0);
      piston_window::text::Text::new_color([0.0, 0.0, 0.0, 1.0], 6).draw(
          &self.state.title_text.current_text(),
          &mut *self.glyphs.borrow_mut(),
          &context.draw_state,
          transform,
          graphics
      ).expect("Failed drawing main story text");

      let transform = context.transform.trans((window_size.width/2 + 200) as f64,
                                              (window_size.height - 35) as f64);
      piston_window::text::Text::new_color([1.0, 1.0, 1.0, 1.0], 3).draw(
          &self.state.hint_text.current_text(),
          &mut *self.glyphs.borrow_mut(),
          &context.draw_state,
          transform,
          graphics
      ).expect("Failed drawing hint text");
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
  Rc::new(RefCell::new(default_actor::DefaultActor::new(actor, assets, scene.clone())))
}

fn make_hero(
  actor: &level::Hero,
  assets: &assets::AssetMap,
  scene: SceneRcRef,
) -> hero::HeroRcRef {
  Rc::new(RefCell::new(hero::Hero::new(actor, assets, scene.clone())))
}

fn make_detective(
  actor: &level::Detective,
  assets: &assets::AssetMap,
  scene: SceneRcRef,
) -> detective::DetectiveRcRef {
  Rc::new(RefCell::new(detective::Detective::new(actor, assets, scene.clone())))
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

    // Load assets
    let assets = assets::load_assets(&mut window.borrow_mut());

    // Load font
    let glyphs = font::load_font(String::from("Pixel-Noir.ttf"), &mut window.borrow_mut());

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

    // insert detective
    let detective_cfg = level.detective;
    let detective = make_detective(&detective_cfg, &assets, scene.clone());
    state.detective = Some(detective.clone());
    state.entities.insert(detective_cfg.name.to_owned(), detective);

    // insert hero
    let hero_cfg = level.hero;
    let hero = make_hero(&hero_cfg, &assets, scene.clone());
    state.hero = Some(hero.clone());
    state.entities.insert(hero_cfg.name.to_owned(), hero);


    let mut sound_effects = sound::SoundEffects::new();
    sound_effects.start_music();

    GameMode::new_with_state(window,
                             state,
                             scene.clone(),
                             sound_effects,
                             glyphs)
  }

  /// Create a GameMode with an existing State.
  pub fn new_with_state(
    window: Rc<RefCell<piston_window::PistonWindow<Window>>>,
    state: State,
    //assets: assets::AssetMap,
    scene: SceneRcRef,
    sound_effects: sound::SoundEffects,
    glyphs: Rc<RefCell<piston_window::Glyphs>>,
  ) -> GameMode<Window> {
    GameMode {
      window,
      state,
      //assets,
      scene,
      sound_effects,
      glyphs,
    }
  }
}
