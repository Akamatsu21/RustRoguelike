use tcod::colors::Color;
use tcod::console::{Console, BackgroundFlag};
use crate::game::Game;

#[derive(Debug)]
pub struct Entity
{
    x: i32,
    y: i32,
    c: char,
    color: Color
}

impl Entity
{
    pub fn new(x: i32, y: i32, c: char, color: Color) -> Self
    {
        Entity {x, y, c, color}
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game)
    {
        if game.map[(self.x + dx) as usize][(self.y + dy) as usize].passable
        {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, canvas: &mut dyn Console)
    {
        canvas.set_default_foreground(self.color);
        canvas.put_char(self.x, self.y, self.c, BackgroundFlag::None);
    }
}