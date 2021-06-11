/*use specs::prelude::*;
use specs_derive::Component;
use sdl2::rect::{Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction
{
    Up,
    Down,
    Left,
    Right,
}

/* DEFINE SOME COMPONENTS */
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity
{
    pub speed: i32,
    pub direction: Direction,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite
{
    pub spritesheet: usize, // Spritesheet to render from
    pub region: Rect, // Region of spritesheet
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation
{
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}
*/