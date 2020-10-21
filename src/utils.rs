use crate::constants::{BaseType, BASE_UNIT, GRID_SIZE};

use bevy::prelude::{Transform, Vec2, Vec3};
use grid::Grid;
use num_traits::Pow;
use rand::{seq::IteratorRandom, thread_rng};

pub fn convert_to_unit(val: f32) -> BaseType {
    (val / BASE_UNIT) as BaseType
}

pub fn convert_from_unit(unit: BaseType) -> f32 {
    (unit as f32) * BASE_UNIT
}

pub fn convert_vec2_to_unit(vec2: Vec2) -> (BaseType, BaseType) {
    (convert_to_unit(vec2.x()), convert_to_unit(vec2.y()))
}

pub fn calculate_move_cost(distance: f32) -> f32 {
    distance.pow(1.2) / 80.0
}

pub fn is_out_of_box(translation: Vec3) -> bool {
    let max_x = (GRID_SIZE.0 / 2) as f32;
    let max_y = (GRID_SIZE.1 / 2) as f32;
    let min_x = -max_x;
    let min_y = -max_y;

    if translation.x() > min_x
        && translation.x() < max_x
        && translation.y() > min_y
        && translation.y() < max_y
    {
        false
    } else {
        true
    }
}

pub fn calculate_random_objects(
    object_width: usize,
    object_height: usize,
    count: usize,
    translations: impl Iterator<Item = Vec3>,
) -> impl Iterator<Item = Transform> {
    let mut rng = thread_rng();

    let mut grid = Grid::new(GRID_SIZE.1 / object_height, GRID_SIZE.0 / object_width);

    for translation in translations {
        let x = (((GRID_SIZE.0 / 2) as f32 + translation.x()).max(0.0) as usize) / object_width;
        let x = x.min(grid.cols() - 1);
        let y = (((GRID_SIZE.1 / 2) as f32 + translation.y()).max(0.0) as usize) / object_height;
        let y = y.min(grid.rows() - 1);

        grid[y][x] = true;
    }

    let min_x = -(((GRID_SIZE.0 / 2) - object_width) as f32);
    let min_y = -(((GRID_SIZE.1 / 2) - object_height) as f32);

    grid.iter_mut()
        .enumerate()
        .filter_map(|(i, v)| if *v { None } else { Some(i) })
        .choose_multiple(&mut rng, count)
        .into_iter()
        .map(move |idx| {
            let y = idx / grid.cols();
            let x = idx - y * grid.cols();
            Transform::from_translation(Vec3::new(
                ((x * object_width) as f32 - (GRID_SIZE.0 / 2) as f32).max(min_x),
                ((y * object_height) as f32 - (GRID_SIZE.1 / 2) as f32).max(min_y),
                0.0,
            ))
        })
}
