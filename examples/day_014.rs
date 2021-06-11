/*
    100 DAYS OF CODE : Day 13
    SDL 2 Game

    Today I started with setting up SDL2 and Rust.
    I found a nice tutorial that goes over a simple game with SDL2.
*/
/*
use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
    rect::{Point, Rect},
    render::{WindowCanvas, Texture},
    image::{self, LoadTexture, InitFlag},
};

use std::time::Duration;

use one_hundred_days_of_code::{
    components::*,
    physics,
    animator,
};

const FPS_CAP: u32 = 30;
const PLAYER_MOVEMENT_SPEED: i32 = 20;

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

fn character_animation_frames(spritesheet: usize, top_left_frame: Rect,
    direction: Direction) -> Vec<Sprite>
{
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3
    {
        frames.push(Sprite
        {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height
            ),
        });
    }

    frames
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

    let mut dispatcher = DispatcherBuilder::new()
        .with(physics::Physics, "Physics", &[])
        .with(animator::Animator, "Animator", &[])
        .build();

    let textures = [
        texture_creator.load_texture("assets/bardo.png")?,
    ];

    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 26, 36);

    let player_animation = MovementAnimation
    {
        current_frame: 0,
        up_frames: character_animation_frames(player_spritesheet,
            player_top_left_frame, Direction::Up),
        down_frames: character_animation_frames(player_spritesheet,
            player_top_left_frame, Direction::Down),
        left_frames: character_animation_frames(player_spritesheet,
            player_top_left_frame, Direction::Left),
        right_frames: character_animation_frames(player_spritesheet,
            player_top_left_frame, Direction::Right),
    };

    let mut world = World::new();
    dispatcher.setup(&mut world.res);

    world.create_entity()
        .with(Position(Point::new(0,0)))
        .with(Velocity { speed: 0, direction: Direction::Right })
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();

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
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;

        // Time management | ~60fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS_CAP));
    }

    Ok(())
}
*/
fn main() {}