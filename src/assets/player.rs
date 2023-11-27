use std::f32::consts::PI;

use sdl2::{render::WindowCanvas, pixels::Color, rect::Point};
use crate::core::{video::{Renderable, Updatable}, constants::{WINDOW_WIDTH, FPS, WINDOW_HEIGHT}};
use crate::core::controller::Controller;
use crate::assets::models::Dynamics;

const PI_2_3: f32 = PI * 2.0 / 3.0;
const TWO_PI: f32 = 2.0 * PI;

const THRUST_PER_FRAME: f32 = 0.025;



pub struct Player {
    dynamics: Dynamics,
}

impl Player {
    pub fn new() -> Player {
        Player {
            dynamics: Dynamics { 
                position: ((WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32), 
                heading: 0.0, 
                speed_vector: (0.0, 0.0), 
                thrust: 0.0 
            },
        }
    }
}

impl Renderable for Player {
    fn render(&self, canvas : &mut WindowCanvas) {
        canvas.set_draw_color(Color::WHITE);

        let xf32 : f32 = self.dynamics.position.0 as f32;        
        let yf32 : f32 = self.dynamics.position.1 as f32;        

        let x_a = xf32 + 20.0 * self.dynamics.heading.cos();
        let y_a = yf32 + 20.0 * self.dynamics.heading.sin();
        let x_b = xf32 + 10.0 * (self.dynamics.heading - PI_2_3).cos();
        let y_b = yf32 + 10.0 * (self.dynamics.heading - PI_2_3).sin();
        let x_c = xf32 + 10.0 * (self.dynamics.heading + PI_2_3).cos();
        let y_c = yf32 + 10.0 * (self.dynamics.heading + PI_2_3).sin();

        let _ = canvas.draw_lines([
            Point::new(x_a as i32, y_a as i32), 
            Point::new(x_b as i32, y_b as i32), 
            Point::new(x_c as i32, y_c as i32),
            Point::new(x_a as i32, y_a as i32)
            ]
        .as_slice());
    }
}

impl Updatable for Player {
    fn update_state(&mut self, controller: &mut Controller) {
        
        //thrust
        if controller.is_key_pressed(sdl2::keyboard::Keycode::Up) {
            self.dynamics.thrust += THRUST_PER_FRAME;
        } else {
            self.dynamics.thrust = 0.0;
        }
        self.dynamics.thrust = if self.dynamics.thrust < 0.0 {0.0} else if self.dynamics.thrust > 100.0 {100.0} else {self.dynamics.thrust};

        //heading
        if controller.is_key_pressed(sdl2::keyboard::Keycode::Left) {
            self.dynamics.heading -= 0.05;
        } else if controller.is_key_pressed(sdl2::keyboard::Keycode::Right) {
            self.dynamics.heading += 0.05;
        } else if controller.is_key_pressed_once(sdl2::keyboard::Keycode::Down) {
            self.dynamics.heading += PI;
        } else if controller.is_key_pressed(sdl2::keyboard::Keycode::Space) {
            self.fire();
        }
        self.dynamics.heading = if self.dynamics.heading < 0.0 {self.dynamics.heading + TWO_PI} 
            else if self.dynamics.heading > TWO_PI {self.dynamics.heading - TWO_PI} 
            else {self.dynamics.heading};



        //compute speed vector
        let speed_pps = FPS as f32 * (self.dynamics.thrust / 100.0);
        self.dynamics.speed_vector.0 = (self.dynamics.speed_vector.0 + speed_pps * self.dynamics.heading.cos()).min(10.0);
        self.dynamics.speed_vector.1 = (self.dynamics.speed_vector.1 + speed_pps * self.dynamics.heading.sin()).min(10.0);
       
        self.dynamics.position.0 += self.dynamics.speed_vector.0;
        self.dynamics.position.1 += self.dynamics.speed_vector.1;
        self.dynamics.position.0 = if self.dynamics.position.0 > WINDOW_WIDTH as f32 {0.0} 
            else if self.dynamics.position.0 < 0.0 {WINDOW_WIDTH as f32} 
            else {self.dynamics.position.0};
        self.dynamics.position.1 = if self.dynamics.position.1 > WINDOW_HEIGHT as f32 {0.0} 
            else if self.dynamics.position.1 < 0.0 {WINDOW_HEIGHT as f32} 
            else {self.dynamics.position.1};

        
        
        
    }
}

impl Player {
    pub fn fire(&mut self) {
        // if self.active_fire_count > 5.0 {
        //     return;
        // }

    }
}