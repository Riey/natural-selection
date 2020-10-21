use crate::component::{SimulationUi, Wall};
use crate::constants::{GRID_BOUND, SCALE_F};
use crate::resource::GameSprites;

use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let creature_texture = asset_server
        .load("creature.png");
    let creature_filled_texture = asset_server
        .load("creature_filled.png");
    let food_texture = asset_server
        .load("food.png");
    let font = asset_server
        .load("Hack-Regular.ttf");

    let mut camera = Camera2dComponents::default();
    camera.transform.scale = Vec3::splat(SCALE_F);
    camera.orthographic_projection.far *= SCALE_F;

    commands
        .insert_resource(GameSprites::new(
            &mut materials,
            creature_texture,
            creature_filled_texture,
            food_texture,
        ))
        .spawn(camera)
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            text: Text {
                value: "TURN: 0, FPS: 0.0".to_string(),
                font,
                style: TextStyle {
                    color: Color::WHITE,
                    font_size: 30.0,
                },
            },
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(SimulationUi);

    let wall_material = materials.add(Color::WHITE.into());
    let thickness = 10.0;
    let bound = GRID_BOUND + Vec2::splat(thickness * 2.0);

    commands
        .spawn(SpriteComponents {
            material: wall_material.clone_weak(),
            transform: Transform::from_translation(Vec3::new(-bound.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(thickness, bound.y() + thickness)),
            ..Default::default()
        })
        .with(Wall)
        .spawn(SpriteComponents {
            material: wall_material.clone_weak(),
            transform: Transform::from_translation(Vec3::new(bound.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(thickness, bound.y() + thickness)),
            ..Default::default()
        })
        .with(Wall)
        .spawn(SpriteComponents {
            material: wall_material.clone_weak(),
            transform: Transform::from_translation(Vec3::new(0.0, -bound.y() / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bound.x() + thickness, thickness)),
            ..Default::default()
        })
        .with(Wall)
        .spawn(SpriteComponents {
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(0.0, bound.y() / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bound.x() + thickness, thickness)),
            ..Default::default()
        })
        .with(Wall);
}
