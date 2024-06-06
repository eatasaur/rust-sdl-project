use std::sync::atomic::{AtomicUsize, Ordering};
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

use crate::model::game::BoardPiece;
use crate::player::player_rect::PlayerRect;
use crate::utils::key_manager::KeyManager;

const ROW_COL_SIZE: i32 = 5;
const MAX_RGB: u8 = 255;

pub struct Renderer {
    pub screen_area: Rect,
    pub screen_color: Color,
}

impl Renderer {

    pub fn render(
        &mut self, 
        canvas: &mut Canvas<Window>,
        board: &[[BoardPiece; 5]; 5],
        player_rect: &PlayerRect,
        key_manager: &KeyManager) {
        
            // Initialze syncronous static count
            static COUNT: AtomicUsize = AtomicUsize::new(0);
            
            // Modify screen colour per frame
            self.screen_color.r = COUNT.load(Ordering::Relaxed) as u8;
            self.screen_color.b = MAX_RGB - COUNT.load(Ordering::Relaxed) as u8;

            // Add to the static counter, validate it does not exceed MAX_RGB
            COUNT.fetch_add(1, Ordering::Relaxed);
            COUNT.fetch_and(MAX_RGB as usize, Ordering::Relaxed);
            
            // Draw background
            canvas.set_draw_color(self.screen_color);
            canvas.fill_rect(self.screen_area).ok().unwrap_or_default();

            self.draw_lines(canvas);

            self.draw_pieces(canvas, board);

            self.draw_player(canvas, player_rect);

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn draw_lines(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let cell_width = self.screen_area.w / ROW_COL_SIZE;
        let cell_height = self.screen_area.h / ROW_COL_SIZE;
        
        for i in 0..ROW_COL_SIZE {
            //horizontal
            canvas.draw_line(
                Point::new(cell_width / 2, cell_height / 2 + i * cell_height),
                Point::new(self.screen_area.w - cell_width / 2, cell_height / 2 + i * cell_height)
            ).ok().unwrap_or_default();

            //vertical
            canvas.draw_line(
                Point::new(cell_width / 2 + i * cell_width, cell_height / 2),
                Point::new(cell_width / 2 + i * cell_width, self.screen_area.h - cell_height / 2)
            ).ok().unwrap_or_default();

            //diagonal up-right a
            canvas.draw_line(
                Point::new(cell_width / 2, cell_height / 2 + i * cell_height), 
                Point::new(cell_width / 2 + i * cell_width, cell_height / 2)
            ).ok().unwrap_or_default();

            //diagonal up-right b
            canvas.draw_line(
                Point::new(cell_width / 2 + i * cell_width, self.screen_area.h - cell_height / 2), 
                Point::new(self.screen_area.w - cell_width / 2, cell_height / 2 + i * cell_height)
            ).ok().unwrap_or_default();

            //diagonal up-down a
            canvas.draw_line(
                Point::new(cell_width / 2, cell_height / 2 + i * cell_height), 
                Point::new(self.screen_area.w - (cell_width / 2 + i * cell_width), self.screen_area.h - cell_height / 2)
            ).ok().unwrap_or_default();

            //diagonal up-down b
            canvas.draw_line(
                Point::new(cell_width / 2 + i * cell_width, cell_height / 2), 
                Point::new(self.screen_area.w - cell_width / 2, self.screen_area.h - (cell_height / 2 + i * cell_height))
            ).ok().unwrap_or_default();
        }
    }

    pub fn draw_pieces(
        &self, 
        canvas: &mut Canvas<Window>, 
        board: &[[BoardPiece; ROW_COL_SIZE as usize]; ROW_COL_SIZE as usize]) {

            let width: i32 = self.screen_area.w / ROW_COL_SIZE;
            let height: i32 = self.screen_area.h / ROW_COL_SIZE;

            for i in 0i32..ROW_COL_SIZE {
                let row: usize = i.try_into().unwrap();
                for j in 0i32..ROW_COL_SIZE {
                    let col: usize = j.try_into().unwrap();

                    let color: Color;
                    match board[row][col] {
                        BoardPiece::Black => color = Color::RGB(0, 0, 0),
                        BoardPiece::Red => color = Color::RGB(255, 0, 0),
                        BoardPiece::None => continue,
                    }
                    canvas.set_draw_color(color);

                    let rect: Rect = Rect::new(
                        width / 4 + width * j, height / 4 + height * i,
                        (width / 2).try_into().unwrap(), 
                        (height/ 2).try_into().unwrap()
                    );
                    canvas.fill_rect(rect).ok().unwrap_or_default();
                }
            }
    }

    fn draw_player(&self, canvas: &mut Canvas<Window>, player_rect: &PlayerRect) {
        canvas.set_draw_color(player_rect.get_player_color());
        canvas.fill_rect(player_rect.get_player_body()).unwrap();
    }
}