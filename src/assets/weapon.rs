use crate::core::video::{Updatable, Renderable};

use crate::assets::{missile_basic::BasicMissile, missile_triple::TripleMissile};


pub enum Weapon {
    BasicMissile(BasicMissile),
    TripleMissile(TripleMissile)
}


impl Updatable for Weapon {
    fn update_state(&mut self, controller: &mut crate::core::controller::Controller) {

        // let missile_1 = BasicMissile {
        //     weapon: WeaponData { dynamics: Dynamics { position: (0.0,0.0), heading: 0, speed_vector: (0.0, 0.0), thrust: 0.0}, power: 0, duration: 0 }};

        // let mut v : Vec<Weapon> = Vec::new();
        // v.push(Weapon::BasicMissile(missile_1));
        // v.push(Weapon::TripleMissile(missile_1));
        
    }
}
