use crate::models::{Bomb, Fire, Player, Wall};

const SPEED: f64 = 200.0;
const GRID: f64 = 40.0;

pub struct World {
    pub players: Vec<Player>,
    pub walls: Vec<Wall>,
    pub bombs: Vec<Bomb>,
    pub fires: Vec<Fire>,
    pub width: f64,
    pub height: f64,
}

impl World {
    pub fn new(width: f64, height: f64) -> World {
        World {
            players: vec![
                Player::new(0, width - 60.0, height - 60.0, SPEED),
                Player::new(1, 60.0, 60.0, SPEED),
            ],
            walls: World::wall_generate(width, height),
            bombs: vec![],
            fires: vec![],
            width: width,
            height: height,
        }
    }

    pub fn wall_generate(width: f64, height: f64) -> Vec<Wall> {
        let mut wall = vec![];
        let mut i = 20.0;
        loop {
            let mut j = 20.0;
            if i == 20.0 || i == height - 20.0 {
                loop {
                    wall.push(Wall::new(j, i));
                    j += GRID;
                    if j > width {
                        break;
                    }
                }
            } else if (i + 20.0) % 80.0 == 0.0 {
                wall.push(Wall::new(20.0, i));
                wall.push(Wall::new(width - 20.0, i));
            } else if (i + 20.0) % 80.0 != 0.0 {
                loop {
                    wall.push(Wall::new(j, i));
                    j += 80.0;
                    if j > width {
                        break;
                    };
                }
            }
            i += GRID;
            if i > height {
                break;
            }
        }
        wall
    }
}
