use sdl2::render::WindowCanvas;

use crate::core::{video::{Renderable, Updatable}, controller::Controller};

use crate::assets::models::WeaponData;

pub struct BasicMissile {
    weapon: WeaponData,
}

impl BasicMissile {
    fn new() -> BasicMissile {
        todo!()
    }
}

impl Renderable for BasicMissile {
    fn render(&self, canvas : &mut WindowCanvas) {
        todo!()
    }
}

impl Updatable for BasicMissile {
    fn update_state(&mut self, controller: &mut Controller) {
        todo!()
    }
}