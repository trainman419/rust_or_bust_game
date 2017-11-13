extern crate piston_window;
extern crate find_folder;

use std::cmp;
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
  strings: Vec<String>,
  start_time: SystemTime,
  duration: Duration,
}

impl FontTransition {
  pub fn new(strings: Vec<String>, duration: u64)
  -> FontTransition {
    let now = SystemTime::now();
    FontTransition {
      strings: strings,
      start_time: now,
      duration: Duration::new(duration, 0),
    }
  }

  pub fn next(&mut self, next_text: String, duration: u64) {
    let now = SystemTime::now();
    self.strings.push(next_text);
    self.start_time = now;
    self.duration = Duration::new(duration, 0);
  }

  pub fn current_text(&self) -> String {
    let now = SystemTime::now();
    // If it's time to change the text
    if self.start_time < now {
      let dt = now.duration_since(self.start_time)
          .expect("You don't understand the time library")
          .as_secs();
      let idx: u64 = dt / self.duration.as_secs();
      let idx = cmp::min(idx, self.strings.len() as u64 - 1);
      self.strings[idx as usize].clone()
    } else {
      self.strings[0].clone()
    }
  }

}
