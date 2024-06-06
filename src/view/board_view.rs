use std::sync::atomic::{AtomicUsize, Ordering};
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use specs::{World, WorldExt, Join};
use std::time::Duration;

use crate::game::{Position, Renderable};

use crate::utils;
use utils::texture_manager::TextureManager;

const MAX_RGB: u8 = 255;

// Initialze syncronous static count
static COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Renderer {
    pub screen_area: Rect,
    pub screen_color: Color,
}

impl Renderer {

    pub fn render(
        &mut self, 
        canvas: &mut Canvas<Window>,
        texture_manager: &mut TextureManager<WindowContext>,
        _texture_creator: &TextureCreator<WindowContext>,
        _font: &sdl2::ttf::Font,
        ecs: &World) -> Result<(), String>{
            canvas.clear();
            
            // Modify screen colour per frame
            self.screen_color.r = COUNT.load(Ordering::Relaxed) as u8;
            self.screen_color.b = MAX_RGB - COUNT.load(Ordering::Relaxed) as u8;

            // Add to the static counter, validate it does not exceed MAX_RGB
            COUNT.fetch_add(1, Ordering::Relaxed);
            COUNT.fetch_and(MAX_RGB as usize, Ordering::Relaxed);
            
            // Draw background
            canvas.set_draw_color(self.screen_color);
            canvas.fill_rect(self.screen_area).ok().unwrap_or_default();

            let positions = ecs.read_storage::<Position>();
            let renderables = ecs.read_storage::<Renderable>();

            for (renderable, pos) in (&renderables, &positions).join() {
                let src = Rect::new(0, 0, renderable.i_w, renderable.i_h);

                let x: i32 = pos.x as i32;
                let y: i32 = pos.y as i32;
                let dst = Rect::new(x - ((renderable.o_w / 2) as i32), y - ((renderable.o_h / 2) as i32), renderable.o_w, renderable.o_h);

                let center = Point::new((renderable.o_w / 2) as i32, (renderable.o_h / 2 ) as i32);
                let texture = texture_manager.load(&renderable.tex_name)?;

                canvas.copy_ex(
                    &texture, 
                    src, 
                    dst, 
                    pos.rot, 
                    center, 
                    false, 
                    false)?;
            }

            canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));

            Ok(())
    }
}