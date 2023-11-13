
use std::f32::consts::PI;

use sdl2::{pixels::Color, render::WindowCanvas, rect::{Rect, Point}};
use crate::core::{video::{Renderable, Updatable}, constants::{WINDOW_WIDTH, self}};

const LISA_COUNT: u16 = 6;

#[derive(Clone)]
struct Lisa {
    speed: f32,      //rounds per sec
    angle: f32,
    center_x: i32,
    center_y: i32,
    size: i32,
    color: Color,
}


impl Renderable for Lisa {
     fn render(&self, canvas : &mut WindowCanvas) {
        //draw border
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_rect(Rect::from_center(Point::from((self.center_x as i32, self.center_y as i32)), 
            self.size as u32, self.size as u32))
            .expect("Error rendering LISA");
        
        // draw lisa
        canvas.set_draw_color(self.color);
        let half_size = self.size >> 1;
        let px: f32 = (self.center_x) as f32 + (self.angle.cos() * half_size as f32);
        let py: f32 = (self.center_y) as f32 + (self.angle.sin() * half_size as f32);
        canvas.draw_point(Point::from((px as i32, py as i32)))
            .expect("Error rendering LISA");


     }
}

impl Updatable for Lisa {
    fn update_state(&mut self) {
        self.angle -= (2.0 * PI / constants::FPS as f32) * self.speed;
    }

}

pub struct World {
    lisas : Vec<Lisa>
}

impl World {
    pub fn new() -> World {
        log::debug!("Creating world objects");
        let mut world = World {lisas : Vec::new()};

        let margin_size = (WINDOW_WIDTH as f32 / LISA_COUNT as f32 / 10.0) as i32;
        let lisa_size = ((WINDOW_WIDTH as i32 - (margin_size * (LISA_COUNT + 1) as i32)) as f32 / LISA_COUNT as f32) as i32;

        let mut x = margin_size;
        let mut y = margin_size;
        let mut speed = 1.0;
        for iter in 1..=LISA_COUNT * LISA_COUNT {
            world.lisas.push(Lisa {
                speed, 
                angle: 0.0,
                center_x: x + lisa_size / 2, 
                center_y: y + lisa_size / 2, 
                size: lisa_size,
                color: Color::RGB(255 - (iter * 6) as u8, 255 - (iter * 6) as u8, 255 - (iter * 6) as u8)    
            });
            x += lisa_size + margin_size;
            speed -= speed / (LISA_COUNT + 1) as f32;
            if iter % LISA_COUNT == 0 {
                x = margin_size;
                y += lisa_size + margin_size;
                speed = 1.0;
            }
        }
        world
    }

    pub fn update(&mut self) {
        self.lisas.iter_mut().for_each(|l| l.update_state())
    }

    pub fn renderable_items(&mut self) -> Vec<impl Renderable> {
        self.lisas.to_vec()
    }
}
