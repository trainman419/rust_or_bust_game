extern crate serde_json;
extern crate std;

use error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Actor {
  pub name: String,
  pub image: String,
  pub position: Point,
  pub scale: f64,
  pub visible: bool,
  pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trigger {
  pub name: String,
  pub on_hero_interact: String,
  pub on_detective_interact: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Level {
  pub world_bounds: (Point, Point),
  pub actors: Vec<Actor>,
}

impl Level {
  pub fn from_path_str(path_str: &str) -> error::Result<Level> {
    Self::from_path(&std::path::Path::new(path_str))
  }

  pub fn from_path(path: &std::path::Path) -> error::Result<Level> {
    let file = std::fs::File::open(path)?;
    let config = serde_json::from_reader(file)?;
    Ok(config)
  }
}
