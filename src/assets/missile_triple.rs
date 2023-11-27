use sdl2::render::WindowCanvas;

use crate::core::{video::{Renderable, Updatable}, controller::Controller};
use crate::assets::models::WeaponData;

pub struct TripleMissile {
    weapon: WeaponData,
}

impl TripleMissile {
    fn new() -> TripleMissile {
        todo!()
    }
}

impl Renderable for TripleMissile {
    fn render(&self, canvas : &mut WindowCanvas) {
        todo!()
    }
}

impl Updatable for TripleMissile {
    fn update_state(&mut self, controller: &mut Controller) {
        todo!()
    }
}