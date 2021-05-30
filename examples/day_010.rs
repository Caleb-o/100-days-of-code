/*
    100 DAYS OF CODE : Day 10
    SDL 2 Game

    Today I started with setting up SDL2 and Rust.
    I found a nice tutorial that goes over a simple game with SDL2.
*/

use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
    render::{WindowCanvas, Texture},
    image::{self, LoadTexture, InitFlag},
};
use std::time::Duration;

fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture) -> Result<(), String>
{
    canvas.set_draw_color(color);
    canvas.clear();

    canvas.copy(texture, None, None)?;
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
    let texture = texture_creator.load_texture("assets/bardo.png")?;

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
                },
                _ => {}
            }
        }

        // Change rgb colour over time
        i = (i + 1) % 255;
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture)?;

        // Time management | ~60fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}