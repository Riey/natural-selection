use bevy::math::{const_vec2, Vec2};
use bevy::render::color::Color;

pub const BACK_COLOR: Color = Color::BLACK;
pub const GRID_SIZE: (usize, usize) = (800, 300);
pub const GRID_BOUND: Vec2 = const_vec2!([GRID_SIZE.0 as f32, GRID_SIZE.1 as f32]);
