use crate::game_state::GameState;
use crate::models::{Bomb, Fire, Item, Player, SoftBlock, Wall};

const THRESHOLD: f64 = 50.0;
const GRID: f64 = 40.0;

pub struct CollisionsController;

impl CollisionsController {
    pub fn collisions(state: &mut GameState) {
        CollisionsController::handle_player_collisions(state);
        CollisionsController::handle_fire_collisions(state);
    }

    // Player の衝突判定
    fn handle_player_collisions(state: &mut GameState) {
        let players = &mut state.world.players;
        let fires = &state.world.fires;
        let walls = &state.world.walls;
        let bombs = &mut state.world.bombs;
        let sblocks = &state.world.sblocks;
        let items = &mut state.world.items;

        for player in players {
            // Fire との衝突判定
            for fire in fires {
                CollisionsController::collides_with_fire(player, fire);
            }

            // Bomb との衝突判定
            for bomb in &mut *bombs {
                CollisionsController::collides_with_bomb(player, bomb)
            }

            // Wall との衝突判定
            for wall in walls {
                CollisionsController::collides_with_wall(player, wall)
            }

            // sblock との衝突判定
            for sblock in sblocks {
                CollisionsController::collides_with_sblock(player, sblock)
            }

            // items との衝突判定
            for item in &mut *items {
                if (player.x() - item.x()).abs() < 20.0 && (player.y() - item.y()).abs() < 20.0 {
                    if item.item_id == 1 {
                        // ボム
                        if player.bombs_limit < 5 {
                            player.bombs_limit += 1;
                        }
                    } else if item.item_id == 2 {
                        // 火力
                        if player.fire < 5 {
                            player.fire += 1;
                        }
                    } else if item.item_id == 3 {
                        // 速度
                        player.speed += 20.0;
                    }
                }
            }

            // Player と接触した Item を消去
            items.retain(|item| {
                (item.x() - player.x()).abs() >= 20.0 || (item.y() - player.y()).abs() >= 20.0
            });
        }
    }

    // Fire の衝突判定
    fn handle_fire_collisions(state: &mut GameState) {
        let sblocks = &mut state.world.sblocks;
        let fires = &mut state.world.fires;
        let bombs = &mut state.world.bombs;
        let items = &mut state.world.items;

        for fire in fires {
            // Bomb との接触判定
            for bomb in &mut *bombs {
                if fire.x() == bomb.x() && fire.y() == bomb.y() {
                    bomb.ttl = 0.0;
                }
            }

            // SoftBlock との接触判定
            for sblock in &mut *sblocks {
                if fire.x() == sblock.x() && fire.y() == sblock.y() && sblock.item_id != 0 {
                    items.push(Item::new(sblock.x(), sblock.y(), sblock.item_id));
                }
            }

            // 接触した SoftBlock を消去
            sblocks.retain(|sblock| sblock.x() != fire.x() || sblock.y() != fire.y());
        }
    }

    pub fn collides_with_fire(player: &mut Player, fire: &Fire) {
        if (player.x() - fire.x()).abs() < THRESHOLD && (player.y() - fire.y()).abs() < THRESHOLD {
            let dist_x = player.x() - fire.x();
            let dist_y = player.y() - fire.y();
            let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
            if dist < GRID / 2.0 {
                *player.x_mut() = -20.0;
                *player.y_mut() = -20.0;
                player.bombs_limit = 0;
                player.speed = 0.0;
                player.fire = 0;
                player.live = false;
            }
        }
    }

    pub fn collides_with_bomb(player: &mut Player, bomb: &mut Bomb) {
        let player_r = GRID / 2.0;
        if (player.x() - bomb.x()).abs() < THRESHOLD && (player.y() - bomb.y()).abs() < THRESHOLD {
            let dist_x = player.x() - bomb.x();
            let dist_y = player.y() - bomb.y();
            let mut dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
            if dist < GRID {
                if bomb.on_player && player.id == bomb.player_id {
                    bomb.on_player = true;
                } else {
                    if dist == 0.0 {
                        dist = 1.01;
                    }
                    let mv_x = dist_x * (player_r + 20.0 - dist) / dist;
                    let mv_y = dist_y * (player_r + 20.0 - dist) / dist;
                    *player.x_mut() = player.x() + mv_x;
                    *player.y_mut() = player.y() + mv_y;
                }
            } else {
                if bomb.on_player {
                    if player.id == bomb.player_id {
                        bomb.on_player = false;
                    }
                }
            }
        }
    }

    pub fn collides_with_wall(player: &mut Player, wall: &Wall) {
        if (player.x() - wall.x()).abs() < THRESHOLD && (player.y() - wall.y()).abs() < THRESHOLD {
            let mut mv_x = player.x();
            let mut mv_y = player.y();
            if player.x() < wall.x() - GRID / 2.0 {
                mv_x = wall.x() - GRID / 2.0;
            } else if player.x() > wall.x() + GRID / 2.0 {
                mv_x = wall.x() + GRID / 2.0;
            }
            if player.y() < wall.y() - GRID / 2.0 {
                mv_y = wall.y() - GRID / 2.0;
            } else if player.y() > wall.y() + GRID / 2.0 {
                mv_y = wall.y() + GRID / 2.0;
            }
            let dist_x = player.x() - mv_x;
            let dist_y = player.y() - mv_y;
            let mut dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
            if dist <= GRID / 2.0 {
                if dist == 0.0 {
                    dist = 1.01;
                }
                let mv_dist_x = dist_x * (GRID / 2.0 - dist) / dist;
                let mv_dist_y = dist_y * (GRID / 2.0 - dist) / dist;
                *player.x_mut() += mv_dist_x;
                *player.y_mut() += mv_dist_y;
            }
        }
    }

    pub fn collides_with_sblock(player: &mut Player, sblock: &SoftBlock) {
        if (player.x() - sblock.x()).abs() < THRESHOLD
            && (player.y() - sblock.y()).abs() < THRESHOLD
        {
            let mut test_x = player.x();
            let mut test_y = player.y();
            if player.x() < sblock.x() - GRID / 2.0 {
                test_x = sblock.x() - GRID / 2.0;
            } else if player.x() > sblock.x() + GRID / 2.0 {
                test_x = sblock.x() + GRID / 2.0;
            }
            if player.y() < sblock.y() - GRID / 2.0 {
                test_y = sblock.y() - GRID / 2.0;
            } else if player.y() > sblock.y() + GRID / 2.0 {
                test_y = sblock.y() + GRID / 2.0;
            }
            let dist_x = player.x() - test_x;
            let dist_y = player.y() - test_y;
            let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
            if dist <= GRID / 2.0 {
                let mv_dist_x = dist_x * (GRID / 2.0 - dist) / dist;
                let mv_dist_y = dist_y * (GRID / 2.0 - dist) / dist;
                *player.x_mut() = player.x() + mv_dist_x;
                *player.y_mut() = player.y() + mv_dist_y;
            }
        }
    }
}
