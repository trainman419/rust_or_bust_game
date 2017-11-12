extern crate piston_window;

use std::rc::Rc;
use std::collections::HashMap;

type Texture = piston_window::G2dTexture;

pub struct Frame {
    pub texture: Rc<Texture>,
    pub frame_time: u16, // frame delay, units of 10ms
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

    pub fn add_frame(&mut self, texture: Rc<Texture>, frame_time: u16) {
        self.frames.push(Frame {
            texture,
            frame_time,
        });
    }
}

pub type AssetMap = HashMap<String, ImageAsset>;
