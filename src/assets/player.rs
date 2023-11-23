use std::f32::consts::PI;

use sdl2::{render::WindowCanvas, pixels::Color, rect::Point};
use crate::core::{video::{Renderable, Updatable}, constants::WINDOW_WIDTH};
use crate::core::controller::Controller;

const PI_2_3: f32 = PI * 2.0 / 3.0;

pub struct Player {
    pos_x: u16,         //player position
    pos_y: u16,
    heading: f32,       //heading in rads, 0 ->  PI <-
    speed_x: f32,       //speed in pixels/sec?
    speed_y: f32,
    thrust: f32,        //current thrust 0-100%
    thrust_acc: f32     //thrust acceleration
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos_x : WINDOW_WIDTH / 2,
            pos_y : WINDOW_WIDTH / 2,
            heading: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            thrust: 0.0,
            thrust_acc: 0.0
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
        if controller.is_key_pressed(sdl2::keyboard::Keycode::Up) {
            self.thrust_acc += if self.thrust_acc > 1.0 {0.0} else {0.1};
        }
        if controller.is_key_pressed(sdl2::keyboard::Keycode::Down) {
            self.thrust -= if self.thrust <= 0.0 {0.0} else {self.thrust_acc};
        }
        self.thrust += if self.thrust > 5.0 {0.0} else {self.thrust_acc};


        if controller.is_key_pressed(sdl2::keyboard::Keycode::Left) {
            self.heading -= 0.01;
        }
        if controller.is_key_pressed(sdl2::keyboard::Keycode::Right) {
            self.heading += 0.01;
        }
        self.heading = match self.heading {
            h if h < 0.0 => h + PI,
            h if h > PI => h - PI,
            h => h
        }

    }
}