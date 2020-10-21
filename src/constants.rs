use bevy::math::{const_vec2, Vec2};
use bevy::render::color::Color;

pub const BACK_COLOR: Color = Color::BLACK;
pub const SCALE: usize = 5;
pub const SCALE_F: f32 = SCALE as f32;
pub const GRID_SIZE: (usize, usize) = (800 * SCALE, 300 * SCALE);
pub const GRID_BOUND: Vec2 = const_vec2!([GRID_SIZE.0 as f32, GRID_SIZE.1 as f32]);
pub const BASE_UNIT: f32 = 0.5;
pub type BaseType = usize;
