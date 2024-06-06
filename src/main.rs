use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

mod model;
use model::game::make_blank_board;
use model::game::GameState;

mod player;
use player::player_rect::PlayerRect;

mod utils;
use utils::key_manager::KeyManager;

mod view;
use view::board_view;

fn main() -> Result<(), String>{

    // Window constants.
    let screen_width = 800;
    let screen_height = 600;

    // Set up SDL Context to get Window and build into Canvas
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("My Rusty Game", screen_width, screen_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build().unwrap();

    // Set up the Renderer object to play on.
    let mut board_view = board_view::Renderer {
        screen_area: Rect::new(0, 0, screen_width, screen_height),
        screen_color: Color::RGB(0, 64, 255),
    };

    // Define Teeko Game State.
    let mut game_state = GameState { board: make_blank_board() };
    game_state.print_board();

    // Define movable player character.
    let player_rect = PlayerRect::new(
        "New Player".to_string(), 
        0, 0, 50, 80);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut key_manager = KeyManager::new();

    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },

                Event::MouseButtonDown { 
                    x, y, .. } => {
                        let col: usize = (5 * x / board_view.screen_area.w).try_into().unwrap();
                        let row: usize = (5 * y / board_view.screen_area.h).try_into().unwrap();
                        game_state.handle_click(row, col);
                    }
                
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
        
        board_view.render(&mut canvas, &game_state.board, &player_rect, &key_manager);


        canvas.present();
    }

    Ok(())
}
