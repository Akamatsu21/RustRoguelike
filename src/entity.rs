use tcod::colors::Color;
use tcod::console::{Console, BackgroundFlag};

#[derive(Debug)]
pub struct Entity
{
    name: String,
    x: i32,
    y: i32,
    c: char,
    color: Color,
    pub blocking: bool,
    pub alive: bool
}

impl Entity
{
    pub fn new(name: &str, x: i32, y: i32, c: char, color: Color, blocking: bool) -> Self
    {
        Entity {name: name.to_string(), x, y, c, color, blocking, alive: false}
    }

    pub fn draw(&self, canvas: &mut dyn Console)
    {
        canvas.set_default_foreground(self.color);
        canvas.put_char(self.x, self.y, self.c, BackgroundFlag::None);
    }

    pub fn pos(&self) -> (i32, i32)
    {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: i32, y: i32)
    {
        self.x = x;
        self.y = y;
    }
}