use crate::component::{SimulationUi, Wall};
use crate::constants::{GRID_BOUND, SCALE_F};
use crate::resource::GameSprites;

use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let creature_texture = asset_server.load("creature.png");
    let creature_filled_texture = asset_server.load("creature_filled.png");
    let food_texture = asset_server.load("food.png");
    let font = asset_server.load("Hack-Regular.ttf");

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.scale = Vec3::splat(SCALE_F);
    camera.orthographic_projection.far *= SCALE_F;

    commands.insert_resource(GameSprites::new(
        &mut materials,
        creature_texture,
        creature_filled_texture,
        food_texture,
    ));
    commands.spawn_bundle(camera);
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn()
        .insert(Text::with_section(
            "TURN: 0, FPS: 0.0",
            TextStyle {
                font: font.clone(),
                font_size: 60.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ))
        .insert(SimulationUi);

    let wall_material = materials.add(Color::WHITE.into());
    let thickness = 10.0;
    let bound = GRID_BOUND + Vec2::splat(thickness * 2.0);

    commands
        .spawn()
        .insert(SpriteBundle {
            material: wall_material.clone_weak(),
            transform: Transform::from_translation(Vec3::new(-bound.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(thickness, bound.y + thickness)),
            ..Default::default()
        })
        .insert(Wall);

    commands
        .spawn()
        .insert(SpriteBundle {
            material: wall_material.clone_weak(),
            transform: Transform::from_translation(Vec3::new(bound.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(thickness, bound.y + thickness)),
            ..Default::default()
        })
        .insert(Wall);

    commands
        .spawn()
        .insert(SpriteBundle {
            material: wall_material.clone_weak(),
            transform: Transform::from_translation(Vec3::new(0.0, -bound.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bound.x + thickness, thickness)),
            ..Default::default()
        })
        .insert(Wall);

    commands
        .spawn()
        .insert(SpriteBundle {
            material: wall_material.clone_weak(),
            transform: Transform::from_translation(Vec3::new(0.0, bound.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(thickness, bound.y + thickness)),
            ..Default::default()
        })
        .insert(Wall);
}
