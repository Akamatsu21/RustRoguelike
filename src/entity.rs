use tcod::colors;

use tcod::colors::Color;
use tcod::console::{Console, BackgroundFlag};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EntityClass
{
    Player,
    Monster
}

#[derive(Clone, Debug, PartialEq)]
pub enum MonsterAi
{
    Basic
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stats
{
    pub hp: i32,
    pub max_hp: i32,
    pub power: i32,
    pub defense: i32
}

#[derive(Debug)]
pub struct Entity
{
    x: i32,
    y: i32,
    c: char,
    color: Color,
    pub name: String,
    pub class: EntityClass,
    pub blocking: bool,
    pub alive: bool,
    pub stats: Option<Stats>,
    pub ai: Option<MonsterAi>
}

impl Entity
{
    pub fn new(name: &str, class: EntityClass, x: i32, y: i32, c: char, color: Color, blocking: bool) -> Self
    {
        Entity {name: name.to_string(), class, x, y, c, color, blocking,
                alive: false, stats: None, ai: None}
    }

    fn player_death(&mut self)
    {
        println!("You died!");
        self.c = '%';
        self.color = colors::DARK_RED;
    }

    fn monster_death(&mut self)
    {
        println!("{} is dead!", self.name);
        self.c = '%';
        self.color = colors::DARK_RED;
        self.blocking = false;
        self.stats = None;
        self.ai = None;
        self.name = format!("{}'s corpse", self.name);
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

    pub fn distance_to(&self, entity: &Entity) -> f32
    {
        let dx: i32 = entity.x - self.x;
        let dy: i32 = entity.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn take_damage(&mut self, damage: i32)
    {
        if let Some(stats) = self.stats.as_mut()
        {
            if damage > 0
            {
                stats.hp -= damage;
            }

            if stats.hp <= 0
            {
                self.die();
            }
        }
    }

    pub fn attack(&mut self, enemy: &mut Entity)
    {
        let damage: i32 = self.stats.map_or(0, |stats| stats.power) - enemy.stats.map_or(0, |stats| stats.defense);
        if damage > 0
        {
            println!("{} attacks {} for {} damage.", self.name, enemy.name, damage);
            enemy.take_damage(damage);
        }
        else
        {
            println!("{} attacks but {} managed to defend.", self.name, enemy.name);
        }
    }

    pub fn die(&mut self)
    {
        self.alive = false;
        match self.class
        {
            EntityClass::Player => self.player_death(),
            EntityClass::Monster => self.monster_death()
        }
    }
}