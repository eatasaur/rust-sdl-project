use specs::{Builder, Join, World, WorldExt};

pub use crate::components::{Position, Renderable, Player};
use crate::utils::key_manager::KeyManager;

const ROTATION_SPEED: f64 = 3.0;
const PLAYER_SPEED: f64 = 4.5;

pub fn update(ecs: &mut World, key_manager: &KeyManager) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();

    for (_, pos) in (&players, &mut positions).join() {
        if key_manager.is_key_pressed("D") {
            pos.rot += ROTATION_SPEED;
        }
        if key_manager.is_key_pressed("A") {
            pos.rot -= ROTATION_SPEED;
        }

        if key_manager.is_key_pressed("W") {
            let radians = pos.rot.to_radians();
            
            pos.x += PLAYER_SPEED * radians.sin();
            pos.y -= PLAYER_SPEED * radians.cos();
        }

        match pos.rot {
            r if r >= 360.0 => { pos.rot -= 360.0;},
            r if r < 0.0 => { pos.rot += 360.0 },
            _ => {},
        }

        match pos.x {
            x if x < 0.0 => { pos.x += crate::GAME_WIDTH as f64 },
            x if x > crate::GAME_WIDTH.into() => { pos.x -= crate::GAME_WIDTH as f64 },
            _ => {},
        }

        match pos.y {
            y if y < 0.0 => { pos.y += crate::GAME_HEIGHT as f64 },
            y if y > crate::GAME_HEIGHT.into() => { pos.y -= crate::GAME_HEIGHT as f64 },
            _ => {},
        }
    }
}

pub fn load_world(ecs: &mut World) {
    ecs.create_entity()
        .with(Position { x: 350.0, y: 250.0, rot: 0.0 })
        .with(Renderable {
            tex_name: "img/space_ship.png".to_string(),
            i_w: 100,
            i_h: 100,
            o_w: 100,
            o_h: 100,
            frame: 0,
            total_frames: 1,
            rot: 0.0
        })
        .with(Player{})
        .build();
}