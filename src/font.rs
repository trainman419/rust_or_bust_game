extern crate piston_window;
extern crate find_folder;

use std::time::{Duration, SystemTime};
use std::cell::RefCell;
use std::rc::Rc;

use piston_window::Glyphs;

pub type GlyphsRcRef = Rc<RefCell<piston_window::Glyphs>>;

pub fn load_font<Window>(font_name: String,
  window: &mut piston_window::PistonWindow<Window>,) -> GlyphsRcRef
where Window: piston_window::Window
{
    // Load font
    let font_dir = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets/fonts").unwrap();
    let ref font = font_dir.join(font_name);
    let glyphs = piston_window::Glyphs::new(
        font,
        window.factory.clone(),
        piston_window::TextureSettings::new().mag(piston_window::Filter::Nearest),
    ).unwrap();
    Rc::new(RefCell::new(glyphs))
}

pub struct FontTransition {
  start_text: String,
  end_text: String,
  start_time: SystemTime,
  duration: Duration,
}

impl FontTransition {
  pub fn new(start_text: &str, end_text: &str, duration: u64)
  -> FontTransition {
    let now = SystemTime::now();
    FontTransition {
      start_text: String::from(start_text),
      end_text: String::from(end_text),
      start_time: now,
      duration: Duration::new(duration, 0),
    }
  }

  pub fn next(&mut self, next_text: &str, duration: u64) {
    let now = SystemTime::now();
    self.start_text = self.end_text.clone();
    self.end_text = String::from(next_text);
    self.start_time = now;
    self.duration = Duration::new(duration, 0);
  }

  pub fn current_text(&self) -> String {
    let now = SystemTime::now();
    if (self.start_time + self.duration) > now {
      self.start_text.clone()
    } else {
      self.end_text.clone()
    }
  }

}
