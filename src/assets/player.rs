use std::f32::consts::PI;

use sdl2::{render::WindowCanvas, pixels::Color, rect::Point};
use crate::core::{video::{Renderable, Updatable}, constants::{WINDOW_WIDTH, FPS, self, WINDOW_HEIGHT}};
use crate::core::controller::Controller;

const PI_2_3: f32 = PI * 2.0 / 3.0;

pub struct Player {
    pos_x: f32,         //player position
    pos_y: f32,
    heading: f32,       //heading in rads, 0 ->  PI <-
    speed_x: f32,       //speed in pixels/sec?
    speed_y: f32,
    m_x: f32,
    m_y: f32,
    thrust: f32,        //current thrust 0-100%
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos_x : (WINDOW_WIDTH / 2) as f32,
            pos_y : (WINDOW_WIDTH / 2) as f32,
            heading: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            thrust: 0.0,
            m_x : 0.0,
            m_y: 0.0
        }
    }
}

impl Renderable for Player {
    fn render(&self, canvas : &mut WindowCanvas) {
        canvas.set_draw_color(Color::WHITE);

        let xf32 : f32 = self.pos_x as f32;        
        let yf32 : f32 = self.pos_y as f32;        

        let x_a = xf32 + 20.0 * self.heading.cos();
        let y_a = yf32 + 20.0 * self.heading.sin();
        let x_b = xf32 + 10.0 * (self.heading - PI_2_3).cos();
        let y_b = yf32 + 10.0 * (self.heading - PI_2_3).sin();
        let x_c = xf32 + 10.0 * (self.heading + PI_2_3).cos();
        let y_c = yf32 + 10.0 * (self.heading + PI_2_3).sin();

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
            self.thrust += 0.5;
        } else if controller.is_key_pressed(sdl2::keyboard::Keycode::Down) {
            self.thrust -= 0.5;
        } 
        self.thrust = match self.thrust {
            t if t < 0.0 => 0.0,
            t if t > 100.0 => 100.0,
            t => t
        };

        //heading
        if controller.is_key_pressed(sdl2::keyboard::Keycode::Left) {
            self.heading -= 0.05;
        }
        if controller.is_key_pressed(sdl2::keyboard::Keycode::Right) {
            self.heading += 0.05;
        }
        self.heading = match self.heading {
            h if h < 0.0 => h + 2.0 * PI,
            h if h > 2.0 * PI => h - 2.0 * PI,
            h => h
        };


        //compute
        let speed_pps = FPS as f32 * (self.thrust / 100.0);
        self.speed_x = speed_pps * self.heading.cos();
        self.speed_y = speed_pps * self.heading.sin();


        self.pos_x += self.speed_x;
        self.pos_y += self.speed_y;
        self.pos_x = match self.pos_x {
            x if x > WINDOW_WIDTH as f32 => 0.0,
            x if x < 0.0 => WINDOW_WIDTH as f32,
            x => x
        };
        self.pos_y = match self.pos_y {
            y if y > WINDOW_HEIGHT as f32 => 0.0,
            y if y < 0.0 => WINDOW_HEIGHT as f32,
            y => y
        };
        
        
        
    }
}