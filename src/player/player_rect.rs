use sdl2::rect::Rect;
use sdl2::pixels::Color;


pub struct PlayerRect {
    name: String,
    body: Rect,
    color: Color,
}

impl PlayerRect {
    pub fn new(name: String, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            name: name,
            body: Rect::new(x, y, width, height),
            color: Color::RGB(0,0,0)
        }
    }

    pub fn get_player_body(&self) -> Rect {
        self.body
    }

    pub fn get_player_color(&self) -> Color {
        self.color
    }

    pub fn move_up(&mut self) {
        self.body.set_y(self.body.y() - 10);
    }

    pub fn move_left(&mut self) {
        self.body.set_x(self.body.x() - 10);
    }

    pub fn move_down(&mut self) {
        self.body.set_y(self.body.y() + 10);
    }

    pub fn move_right(&mut self) {
        self.body.set_x(self.body.x() + 10);
    }
}

