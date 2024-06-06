use std::path::Path;

use game::update;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use specs::{World, WorldExt};

mod components;
use components::{Position, Renderable, Player};

mod game;

mod player;

mod utils;
use utils::key_manager::KeyManager;
use utils::texture_manager::TextureManager;

mod view;
use view::board_view;

const GAME_WIDTH: u32 = 800;
const GAME_HEIGHT: u32 = 600;

struct State {
    ecs: World,
}

fn main() -> Result<(), String>{
    // Set up SDL Context to get Window and build into Canvas
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("My Rusty Game", GAME_WIDTH, GAME_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new(&texture_creator);

    texture_manager.load("img/space_ship.png")?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = Path::new(&"fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // Set up the Renderer object to play on.
    let mut board_view = board_view::Renderer {
        screen_area: Rect::new(0, 0, GAME_WIDTH, GAME_HEIGHT),
        screen_color: Color::RGB(0, 64, 255),
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut key_manager = KeyManager::new();

    let mut game_state = State {
        ecs: World::new()
    };
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Player>();

    game::load_world(&mut game_state.ecs);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(key) => {
                            key_manager.key_down(key.to_string())
                        },
                        None => {}
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        Some(key) => {
                            key_manager.key_up(key.to_string())
                        },
                        None => {}
                    }
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        update(&mut game_state.ecs, &key_manager);
        board_view.render(&mut canvas, &mut texture_manager, &texture_creator, &font,  &game_state.ecs)?;
    }

    Ok(())
}
