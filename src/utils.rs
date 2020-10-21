use crate::constants::GRID_SIZE;

use bevy::prelude::{Transform, Vec3};
use grid::Grid;
use rand::{seq::IteratorRandom, thread_rng};

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
        let x = x.min(grid.rows() - 1);
        let y = (((GRID_SIZE.1 / 2) as f32 + translation.y()).max(0.0) as usize) / object_height;
        let y = y.min(grid.cols() - 1);

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
