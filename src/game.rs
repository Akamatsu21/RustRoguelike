mod rect;
mod tile;

use std::cmp;
use tcod::colors;

use rand::Rng;
use rect::Rect;
use tile::Tile;
use crate::entity::Entity;

pub const MAP_HEIGHT: i32 = 45;
pub const MAP_WIDTH: i32 = 80;
const ROOM_MIN_SIZE: i32 = 6;
const ROOM_MAX_SIZE: i32 = 10;
const MAX_ROOMS: i32 = 30;
const ROOM_MAX_MONSTERS: i32 = 3;

pub type Map = Vec<Vec<Tile>>;

pub struct Game
{
    pub map: Map,
    pub entities: Vec<Entity>
}

impl Game
{
    pub fn new() -> Self
    {
        let mut player = Entity::new("Player", 0, 0, '@', colors::WHITE, true);
        player.alive = true;
        Game {map: vec![], entities: vec![player]}
    }

    fn is_tile_blocked(&self, x: i32, y: i32) -> bool
    {
        if !self.map[x as usize][y as usize].passable
        {
            return true;
        }

        self.entities.iter().any(
            |entity|
            {
                entity.pos() == (x, y) && entity.blocking
            }
        )
    }

    fn create_room(&mut self, rect: Rect)
    {
        for x in (rect.x1 + 1)..rect.x2
        {
            for y in (rect.y1 + 1)..rect.y2
            {
                self.map[x][y] = Tile::empty();
            }
        }
    }

    fn create_horizontal_tunnel(&mut self, x1: usize, x2: usize, y: usize)
    {
        let start = cmp::min(x1, x2);
        let end = cmp::max(x1, x2);
        for x in start..(end + 1)
        {
            self.map[x][y] = Tile::empty();
        }
    }

    fn create_vertical_tunnel(&mut self, y1: usize, y2: usize, x: usize)
    {
        let start = cmp::min(y1, y2);
        let end = cmp::max(y1, y2);
        for y in start..(end + 1)
        {
            self.map[x][y] = Tile::empty();
        }
    }

    fn spawn_entities(&mut self, room: Rect)
    {
        let monster_count: i32 = rand::thread_rng().gen_range(0..(ROOM_MAX_MONSTERS + 1));

        for _ in 0..monster_count
        {
            let x = rand::thread_rng().gen_range((room.x1 + 1)..room.x2) as i32;
            let y = rand::thread_rng().gen_range((room.y1 + 1)..room.y2) as i32;

            if !self.is_tile_blocked(x, y)
            {
                let mut monster =
                    if rand::random::<f32>() < 0.8
                    {
                        Entity::new("Orc", x, y, 'o', colors::DESATURATED_GREEN, true)
                    }
                    else
                    {
                        Entity::new("Troll", x, y, 'T', colors::DARKER_GREEN, true)
                    };

                monster.alive = true;
                self.entities.push(monster);
            }
        }
    }

    pub fn get_player(&mut self) -> &mut Entity
    {
        &mut self.entities[0]
    }

    pub fn move_player(&mut self, dx: i32, dy: i32)
    {
        let (x, y) = self.get_player().pos();
        if !self.is_tile_blocked(x + dx, y + dy)
        {
            self.get_player().set_pos(x + dx, y + dy);
        }
    }

    pub fn create_map(&mut self)
    {
        self.map = vec![
            vec![
                Tile::wall();
                MAP_HEIGHT as usize
            ];
            MAP_WIDTH as usize
        ];

        let mut rooms: Vec<Rect> = vec![];
        for _ in 0..MAX_ROOMS
        {
            let w: i32 = rand::thread_rng().gen_range(ROOM_MIN_SIZE..(ROOM_MAX_SIZE + 1));
            let h: i32 = rand::thread_rng().gen_range(ROOM_MIN_SIZE..(ROOM_MAX_SIZE + 1));
            let x: i32 = rand::thread_rng().gen_range(0..(MAP_WIDTH - w));
            let y: i32 = rand::thread_rng().gen_range(0..(MAP_HEIGHT - h));

            let new_room = Rect::new(x as usize, y as usize, w as usize, h as usize);
            if !rooms.iter().any(|room| new_room.collides(room))
            {
                self.create_room(new_room);
                self.spawn_entities(new_room);
                let (center_x, center_y) = new_room.center();

                if rooms.is_empty()
                {
                    self.get_player().set_pos(center_x as i32, center_y as i32);
                }
                else
                {
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                    if rand::random()
                    {
                        self.create_horizontal_tunnel(prev_x, center_x, prev_y);
                        self.create_vertical_tunnel(prev_y, center_y, center_x);
                    }
                    else
                    {
                        self.create_vertical_tunnel(prev_y, center_y, prev_x,);
                        self.create_horizontal_tunnel(prev_x, center_x, center_y);
                    }
                }
             
                rooms.push(new_room);
            }
        }
    }
}