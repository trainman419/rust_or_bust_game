extern crate piston_window;
extern crate find_folder;

use std::cell::RefCell;
use std::rc::Rc;

use piston_window::Glyphs;
use piston_window::PistonWindow;
use piston_window::WindowSettings;

pub type GlyphsRcRef = Rc<RefCell<piston_window::Glyphs>>;

pub fn load_font<Window>(font_name: String,
  mut window: &mut piston_window::PistonWindow<Window>,) -> GlyphsRcRef
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
