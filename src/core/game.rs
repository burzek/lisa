use sdl2::{event::Event, keyboard::Keycode};

use crate::{core::{video::Screen, constants}, assets::world::World};


pub struct Game {
    screen : Screen,
    world : World
}

impl Game {

    pub fn new() -> Result<Game, String> {
        return match Screen::new() {
            Ok(screen) => Ok(Game{screen: screen, world : World::new()}),
            Err(err) => Err(err)
        }
        
    }

    pub fn start_game(&mut self) {
        log::debug!("Starting main game loop");
        //start main game loop
        let mut event_pump = self.screen.get_event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {
                        keycode : Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / constants::FPS));

            self.world.update();
            self.screen.drawWorld(&mut self.world);    

        }
    }
}