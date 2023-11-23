
use crate::core::video::{Renderable, Updatable};
use crate::core::controller::Controller;
use super::player::Player;



pub struct World {
    player: Player
}

impl World {
    pub fn new() -> Result<World, String> {
        let world = World{
            player :Player::new()
        };
        Ok(world)
    }

    pub fn updateWorld(&mut self, controller: &mut Controller) {
        self.player.update_state(controller);
    }


    pub fn renderable_items(&mut self) -> Vec<&impl Renderable> {
        let mut renderable = Vec::new();
        renderable.push(&self.player);
        renderable
    }

}



