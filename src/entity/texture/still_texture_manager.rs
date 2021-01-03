use opengl_graphics::Texture;

use super::texture_manager::TextureManager;

pub struct StillTextureManager {

    texture: Texture,
    flip: bool,

}

impl StillTextureManager {

    pub fn new(texture: Texture, flip: bool) -> Self {
        Self {
            texture: texture,
            flip: flip,
        }
    }

}

impl TextureManager for StillTextureManager {

    fn reset(&mut self) {

    }

    fn update(&mut self) {

    }

    fn idle(&mut self) {

    }

    fn unidle(&mut self) {

    }

    fn texture(&self) -> (&Texture, bool) {
        (&self.texture, self.flip)
    }
}