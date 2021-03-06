use crate::models::{Bomb, Fire, Item, Player, SoftBlock, Wall};
use rand::seq::SliceRandom;

const SPEED: f64 = 100.0;
const BOMB_LIMIT: i32 = 1;

pub struct World {
    pub players: Vec<Player>,
    pub walls: Vec<Wall>,
    pub sblocks: Vec<SoftBlock>,
    pub bombs: Vec<Bomb>,
    pub fires: Vec<Fire>,
    pub items: Vec<Item>,
    pub width: f64,
    pub height: f64,
    pub time: f64,
}

impl World {
    pub fn new(width: f64, height: f64, num_of_player: i32) -> World {
        let mut map = [
            // 0 = No SoftBlock, 1 = Wall, 2 = SoftBlock(Random)
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 1],
            [1, 0, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 0, 1],
            [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
            [1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1],
            [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
            [1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1],
            [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
            [1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1],
            [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
            [1, 0, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 0, 1],
            [1, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        let mut item = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // ボム増加
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // 火力増加
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 速度増加
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // アイテムなし
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, // ブロックなし
        ];
        let mut rng = rand::thread_rng();
        item.shuffle(&mut rng);

        'item: for n in item {
            for i in 0..13 {
                for j in 0..15 {
                    if map[i][j] == 2 {
                        map[i][j] += n;
                        continue 'item;
                    }
                }
            }
        }

        let mut wall = vec![];
        let mut soft_block = vec![];

        for i in 0..13 {
            for j in 0..15 {
                if map[i][j] == 1 {
                    wall.push(Wall::new(j as f64 * 40.0 + 20.0, i as f64 * 40.0 + 20.0));
                } else if map[i][j] == 3 {
                    soft_block.push(SoftBlock::new(
                        j as f64 * 40.0 + 20.0,
                        i as f64 * 40.0 + 20.0,
                        1,
                    ));
                } else if map[i][j] == 4 {
                    soft_block.push(SoftBlock::new(
                        j as f64 * 40.0 + 20.0,
                        i as f64 * 40.0 + 20.0,
                        2,
                    ));
                } else if map[i][j] == 5 {
                    soft_block.push(SoftBlock::new(
                        j as f64 * 40.0 + 20.0,
                        i as f64 * 40.0 + 20.0,
                        3,
                    ));
                } else if map[i][j] == 6 {
                    soft_block.push(SoftBlock::new(
                        j as f64 * 40.0 + 20.0,
                        i as f64 * 40.0 + 20.0,
                        0,
                    ));
                }
            }
        }

        let mut player = vec![];

        match num_of_player {
            1 => player = vec![Player::new(0, 540.0, 460.0, SPEED, BOMB_LIMIT)],
            2 => {
                player = vec![
                    Player::new(0, 540.0, 460.0, SPEED, BOMB_LIMIT),
                    Player::new(1, 60.0, 60.0, SPEED, BOMB_LIMIT),
                ]
            }
            3 => {
                player = vec![
                    Player::new(0, 540.0, 460.0, SPEED, BOMB_LIMIT),
                    Player::new(1, 60.0, 60.0, SPEED, BOMB_LIMIT),
                    Player::new(2, 540.0, 60.0, SPEED, BOMB_LIMIT),
                ]
            }
            4 | _ => {
                player = vec![
                    Player::new(0, 540.0, 460.0, SPEED, BOMB_LIMIT),
                    Player::new(1, 60.0, 60.0, SPEED, BOMB_LIMIT),
                    Player::new(2, 540.0, 60.0, SPEED, BOMB_LIMIT),
                    Player::new(3, 60.0, 460.0, SPEED, BOMB_LIMIT),
                ]
            }
        };

        World {
            players: player,
            walls: wall,
            sblocks: soft_block,
            bombs: vec![],
            fires: vec![],
            items: vec![],
            width: width,
            height: height,
            time: 180.0,
        }
    }
}
