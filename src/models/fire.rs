use crate::geometry::Point;
use std::f64;
use std::i32;

pub struct Fire {
    pub position: Point,
    pub fire_life_time: f64,
    pub fire_num: i32,
}

impl Fire {
    pub fn new(x: f64, y: f64, fire_num: i32) -> Fire {
        Fire {
            position: Point::new(x, y),
            fire_life_time: 1.0,
            fire_num: 1,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.fire_life_time -= dt;
    }

    pub fn x(&self) -> f64 {
        self.position.x
    }

    pub fn y(&self) -> f64 {
        self.position.y
    }
}