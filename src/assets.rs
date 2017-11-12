extern crate piston_window;
extern crate find_folder;
extern crate gif;
extern crate image;
extern crate tiled;

use std::rc::Rc;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

type Texture = piston_window::G2dTexture;

pub struct Frame {
    pub texture: Rc<Texture>,
    pub frame_time: f64, // frame delay, units of 10ms
}

pub struct ImageAsset {
    pub frames: Vec<Frame>,
}

impl ImageAsset {
    pub fn new() -> ImageAsset {
        ImageAsset {
            frames: Vec::new(),
        }
    }

    pub fn add_frame(&mut self, texture: Rc<Texture>, frame_time: f64) {
        self.frames.push(Frame {
            texture,
            frame_time,
        });
    }
}

pub type AssetMap = HashMap<String, Rc<ImageAsset>>;

fn load_assets_from_dir<Window>(
  mut window: &mut piston_window::PistonWindow<Window>,
  dir: &Path,
  prefix: &str,
  mut assets: &mut AssetMap)
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
                  let mut asset = ImageAsset::new();
                  asset.add_frame(texture, 0.0);
                  assets.insert(name, Rc::new(asset));
              }
              "gif" => {
                  use self::gif::Decoder;
                  use self::gif::SetParameter;
                  println!("Loading {}", name);
                  let mut asset = ImageAsset::new();

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

pub fn load_assets<Window>(mut window: &mut piston_window::PistonWindow<Window>) -> AssetMap
where Window: piston_window::Window
{
  let mut assets = HashMap::new();
  // Load assets. This probably isn't the place, but we'll deal with that
  // later.

  let asset_dir = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();

  load_assets_from_dir(&mut window, &asset_dir, "", &mut assets);

  return assets;
}
