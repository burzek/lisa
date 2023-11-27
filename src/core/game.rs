use sdl2::{event::Event, keyboard::Keycode};
use sdl2::gfx::framerate::FPSManager;
use crate::{core::{video::Screen, constants, controller::Controller}, assets::world::World};

pub struct Game {
    screen : Screen,
    world : World,
    controller: Controller
}

impl Game {

    pub fn new() -> Result<Game, String> {
        let screen = Screen::new()?;
        let world = World::new()?;
        let controller = Controller::new()?;
        Ok(Game { screen, world, controller })
    }

    pub fn start_game(&mut self) {
        log::debug!("Starting main game loop");

        let mut fps_manager = FPSManager::new();
        let _ = fps_manager.set_framerate(constants::FPS);



        //start main game loop
        let mut event_pump = self.screen.get_event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {
                        keycode : Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown { 
                        keycode : Some(kc),
                        ..
                     } => self.controller.key_pressed(kc),
                     Event::KeyUp { 
                        keycode : Some(kc),
                        ..
                     } => self.controller.key_released(kc),
                    _ => {}
                }
            }
            
            fps_manager.delay();

            self.world.update_world(&mut self.controller);
            self.screen.draw_world(&mut self.world);    

        }
    }
}