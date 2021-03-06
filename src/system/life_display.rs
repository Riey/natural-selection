use crate::component::Creature;
use crate::resource::GameSprites;
use bevy::prelude::*;

pub fn life_display_system(
    sprites: Res<GameSprites>,
    mut creature_query: Query<(&Creature, &mut Handle<ColorMaterial>)>,
) {
    for (creature, mut sprite) in &mut creature_query.iter() {
        *sprite = if creature.will_die() {
            sprites.creature()
        } else {
            sprites.creature_filled()
        };
    }
}
