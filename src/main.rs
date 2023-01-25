mod entity;
mod game;

use tcod::console::*;
use tcod::colors;
use tcod::system;
use crate::game::*;

use colors::Color;
use crate::entity::Entity;

const FPS_LIMIT: i32 = 20;
const SCREEN_HEIGHT: i32 = 50;
const SCREEN_WIDTH: i32 = 80;

const GROUND_COLOR: Color = Color {r: 50, g: 50, b: 150};
const WALL_COLOR: Color = Color {r: 0, g: 0, b: 100};

struct Graphics
{
    window: Root,
    canvas: Offscreen
}

fn handle_input(graphics: &mut Graphics, game: &Game, player: &mut Entity) -> bool
{
    use tcod::input::{Key, KeyCode};

    match graphics.window.wait_for_keypress(true)
    {
        Key {code: KeyCode::Enter, alt: true, ..} => graphics.window.set_fullscreen(!graphics.window.is_fullscreen()),
        Key {code: KeyCode::Escape, ..} => return true,
        Key {code: KeyCode::Up, ..} => player.move_by(0, -1, game),
        Key {code: KeyCode::Down, ..} => player.move_by(0, 1, game),
        Key {code: KeyCode::Left, ..} => player.move_by(-1, 0, game),
        Key {code: KeyCode::Right, ..} => player.move_by(1, 0, game),
        _ => ()
    }

    false
}

fn render(graphics: &mut Graphics, game: &Game, entities: &[Entity])
{
    for y in 0..MAP_HEIGHT
    {
        for x in 0..MAP_WIDTH
        {
            if game.map[x as usize][y as usize].blocks_sight
            {
                graphics.canvas.set_char_background(x, y, WALL_COLOR, BackgroundFlag::Set);
            }
            else
            {
                graphics.canvas.set_char_background(x, y, GROUND_COLOR, BackgroundFlag::Set);
            }
        }
    }

    for entity in entities
    {
        entity.draw(&mut graphics.canvas);
    }

    blit(&graphics.canvas, (0, 0), (MAP_WIDTH, MAP_HEIGHT),
         &mut graphics.window,(0, 0), 1.0, 1.0);
}

fn main()
{
    let window =
        Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust roguelike")
            .init();
    let canvas = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let mut graphics = Graphics {window, canvas};

    system::set_fps(FPS_LIMIT);

    let player = Entity::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let npc = Entity::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);
    let mut entities = [player, npc];

    let game = Game {map: create_map()};

    while !graphics.window.window_closed()
    {
        graphics.canvas.clear();
        render(&mut graphics, &game, &entities);
        graphics.window.flush();

        if handle_input(&mut graphics, &game, &mut entities[0])
        {
            break;
        }
    }
}
