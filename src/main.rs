mod entity;
mod game;

use tcod::console::*;
use tcod::colors;
use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::system;
use crate::game::*;

use colors::Color;

const FPS_LIMIT: i32 = 20;
const SCREEN_HEIGHT: i32 = 50;
const SCREEN_WIDTH: i32 = 80;

const FOV_ALGORITHM: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

const GROUND_COLOR_DARK: Color = Color {r: 50, g: 50, b: 150};
const GROUND_COLOR_LIGHT: Color = Color {r: 200, g: 180, b: 50};
const WALL_COLOR_DARK: Color = Color {r: 0, g: 0, b: 100};
const WALL_COLOR_LIGHT: Color = Color {r: 130, g: 110, b: 50};

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlayerAction
{
    TurnPass,
    TurnContinue,
    Exit
}

struct Graphics
{
    window: Root,
    canvas: Offscreen,
    fov: FovMap
}

fn handle_input(graphics: &mut Graphics, game: &mut Game) -> PlayerAction
{
    use tcod::input::{Key, KeyCode};
    use PlayerAction::*;

    let key: Key = graphics.window.wait_for_keypress(true);

    match (key, key.text(), game.get_player().alive)
    {
        (Key {code: KeyCode::Enter, alt: true, ..}, _, _) =>
        {
            graphics.window.set_fullscreen(!graphics.window.is_fullscreen());
            TurnContinue
        },
        (Key {code: KeyCode::Escape, ..}, _, _) => Exit,
        (Key {code: KeyCode::Up, ..}, _, true) =>
        {
            game.move_player(0, -1);
            TurnPass
        },
        (Key {code: KeyCode::Down, ..}, _, true) =>
        {
            game.move_player(0, 1);
            TurnPass
        },
        (Key {code: KeyCode::Left, ..}, _, true) =>
        {
            game.move_player(-1, 0);
            TurnPass
        },
        (Key {code: KeyCode::Right, ..}, _, true) =>
        {
            game.move_player(1, 0);
            TurnPass
        }
        _ => TurnContinue
    }
}

fn render(graphics: &mut Graphics, game: &mut Game, fov_recompute: bool)
{
    if fov_recompute
    {
        graphics.fov.compute_fov(game.get_player().pos().0, game.get_player().pos().1,
                                 TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGORITHM);
    }

    for y in 0..MAP_HEIGHT
    {
        for x in 0..MAP_WIDTH
        {
            let visible: bool = graphics.fov.is_in_fov(x, y);
            let passable: bool = game.map[x as usize][y as usize].passable;
            let color: Color = match (visible, passable)
            {
                (true, true) => GROUND_COLOR_LIGHT,
                (true, false) => WALL_COLOR_LIGHT,
                (false, true) => GROUND_COLOR_DARK,
                (false, false) => WALL_COLOR_DARK
            };

            let explored: &mut bool = &mut game.map[x as usize][y as usize].explored;
            if !*explored && visible
            {
                *explored = true;
            }

            if *explored
            {
                graphics.canvas.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    for entity in game.entities.iter()
    {
        if graphics.fov.is_in_fov(entity.pos().0, entity.pos().1)
        {
            entity.draw(&mut graphics.canvas);
        }
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
    let fov = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
    let mut graphics = Graphics {window, canvas, fov};

    system::set_fps(FPS_LIMIT);

    let mut game = Game::new();
    game.create_map();

    for y in 0..MAP_HEIGHT
    {
        for x in 0..MAP_WIDTH
        {
            graphics.fov.set(x, y,
                             !game.map[x as usize][y as usize].blocks_sight,
                             game.map[x as usize][y as usize].passable);
        }
    }

    let mut prev_pos: (i32, i32) = (-1, -1);

    while !graphics.window.window_closed()
    {
        graphics.canvas.clear();
        let fov_recompute: bool = prev_pos != game.get_player().pos();
        render(&mut graphics, &mut game, fov_recompute);
        graphics.window.flush();

        prev_pos = game.get_player().pos();

        if handle_input(&mut graphics, &mut game) == PlayerAction::Exit
        {
            break;
        }
    }
}
