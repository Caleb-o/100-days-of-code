/*
    100 DAYS OF CODE : Day 13
    SDL 2 Game

    Today I started with setting up SDL2 and Rust.
    I found a nice tutorial that goes over a simple game with SDL2.
*/

use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
    rect::{Point, Rect},
    render::{WindowCanvas, Texture},
    image::{self, LoadTexture, InitFlag},
};
use std::time::Duration;

const FPS_CAP: u32 = 30;
const PLAYER_MOVEMENT_SPEED: i32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction
{
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Player
{
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
    current_frame: u32,
}

// Returns the row of the spritesheet corresponding to the direction
fn direction_spritesheet_row(direction: Direction) -> i32
{
    use self::Direction::*;
    match direction
    {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player
) -> Result<(), String>
{
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    // Player animation
    let (frame_width, frame_height) = player.sprite.size();
    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame as i32,
        player.sprite.y() + frame_height as i32 * direction_spritesheet_row(player.direction),
        frame_width,
        frame_height
    );

    let screen_pos = player.position + Point::new(width as i32 / 2,
        height as i32 / 2);
    let screen_rect = Rect::from_center(screen_pos, frame_width, frame_height);

    canvas.copy(texture, current_frame, screen_rect)?;
    canvas.present();

    Ok(())
}

fn update_player(player: &mut Player)
{
    use self::Direction::*;

    match player.direction
    {
        Up =>
        {
            player.position = player.position.offset(0, -player.speed);
        }
        Down =>
        {
            player.position = player.position.offset(0, player.speed);
        }

        Left =>
        {
            player.position = player.position.offset(-player.speed, 0);
        }
        Right =>
        {
            player.position = player.position.offset(player.speed, 0);
        }
    }

    // Update animation if moving
    if player.speed != 0
    {
        // Hack: All anims are only 3 frames, this is hardcoded
        player.current_frame = (player.current_frame + 1) % 3;
    }
}

fn main() -> Result<(), String>
{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("SDL2 Demo", 800, 600)
        .position_centered()
        .build()
        .expect("Could not initialise video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("Could not make canvas");

    // Create a texture loader and texture
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut player = Player
    {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        speed: 0,
        direction: Direction::Right,
        current_frame: 0,
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    // Main game loop
    'running: loop
    {
        // Handle events
        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} =>
                {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } =>
                {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Up;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. }  =>
                {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Down;
                }

                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. }  =>
                {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Left;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. }  =>
                {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Right;
                }

                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } =>
                {
                    player.speed = 0;
                }

                _ => {}
            }
        }

        // Change rgb colour over time
        i = (i + 1) % 255;
        update_player(&mut player);

        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;

        // Time management | ~60fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS_CAP));
    }

    Ok(())
}
