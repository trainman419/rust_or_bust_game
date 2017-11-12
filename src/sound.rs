#[cfg(unix)]
extern crate ears;

use std::thread;
use std::time;
#[cfg(unix)]
use self::ears::{Sound, Music, AudioController};

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
        let path = String::from("assets/sounds/music/background_theme.wav");
        let handle = thread::spawn(move || {
          let mut music = Music::new(&path).unwrap();
          music.set_looping(true);
          music.set_volume(0.75);
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
          volume = 0.1;
      },
      "crow_squawk" => filename = "crow_squawk.wav",
      "crunchy_leaf" => filename = "crunchy_leaf.wav",
      "foliage_rustle" => {
          filename = "foliage_rustle.wav";
          max_length = 2000;
      },
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
