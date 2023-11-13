use sdl2::{render::WindowCanvas, Sdl, EventPump};
use crate::core::constants;
use crate::assets::world::World;

pub trait Renderable {
    fn render(&self, canvas : &mut WindowCanvas);
}

pub trait Updatable {
    fn update_state(&mut self);
}

pub struct Screen {
    pub context: Sdl, 
    pub canvas: WindowCanvas 
}

impl Screen {
    pub fn new() -> Result<Screen, String> {

        let context = match sdl2::init() {
            Ok(sdl) => sdl,
            Err(_) => {
                panic!("Nepodarilo sa inicializovat SDL2: {}", sdl2::get_error());
            }
        };
        let video_subsystem = match context.video() {
            Ok(video) => video,
            Err(_) => panic!("Nepodarilo sa inicializovat SDL2: {}", sdl2::get_error())
        };
    
        let window = match video_subsystem
            .window("Lisa ", constants::WINDOW_WIDTH as u32, constants::WINDOW_HEIGHT as u32)
            .position_centered()
            .opengl()
            .build() {
                Ok(wnd) => wnd,
                Err(_) => panic!("Nepodarilo sa otvorit okno aplikacie: {}", sdl2::get_error())
            };


        let canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(_) => panic!("Chyba pri zobrazeni: {}", sdl2::get_error())
        };

        Ok(Screen {context, canvas})
    }

    pub fn drawWorld(&mut self, world: &mut World) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255,255,0));
        for r_item in world.renderable_items() {
            r_item.render(&mut self.canvas);
        }
        self.canvas.present();
    }

    pub fn get_event_pump(&mut self) -> Result<EventPump, String> {
        self.context.event_pump()
    }


    
}
