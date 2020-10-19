use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

const BACK_COLOR: Color = Color::BLACK;

fn main() {
    App::build()
        .add_default_plugins()
        .add_resource(ClearColor(BACK_COLOR))
        .add_resource(SimulationState::prepare(2, 20))
        .add_resource(TurnTimer(Timer::from_seconds(5.0, true)))
        .add_resource(TurnCount(0))
        .add_startup_system(setup.system())
        .add_system(simulation.system())
        .add_system(eat_system.system())
        .add_system(movement_system.system())
        .add_system(turn_system.system())
        .add_system(life_display_system.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let creature_texture = asset_server
        .load("assets/creature.png")
        .expect("Load creature texture");
    let creature_filled_texture = asset_server
        .load("assets/creature_filled.png")
        .expect("Load creature filled texture");
    let food_texture = asset_server
        .load("assets/food.png")
        .expect("Load food texture");

    commands
        .insert_resource(GameSprites {
            creature: materials.add(creature_texture.into()),
            creature_filled: materials.add(creature_filled_texture.into()),
            food: materials.add(food_texture.into()),
        })
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());
}

struct GameSprites {
    creature: Handle<ColorMaterial>,
    creature_filled: Handle<ColorMaterial>,
    food: Handle<ColorMaterial>,
}

enum SimulationState {
    Prepare {
        creature_count: usize,
        food_count: usize,
    },
    Running,
}

impl SimulationState {
    pub fn prepare(creature_count: usize, food_count: usize) -> Self {
        SimulationState::Prepare {
            creature_count,
            food_count,
        }
    }

    pub fn running() -> Self {
        SimulationState::Running
    }

    pub fn is_running(&self) -> bool {
        match self {
            SimulationState::Running => true,
            _ => false,
        }
    }
}

struct TurnTimer(Timer);
struct TurnCount(usize);

struct Creature {
    life: usize,
    velocity: Vec2,
}

impl Creature {
    pub fn new() -> Self {
        Self {
            life: 0,
            velocity: Vec2::new(0.0, 0.0),
        }
    }

    pub fn eat_food(&mut self) {
        self.life += 1;
    }

    pub fn time_pass(&mut self) {
        self.life = self.life.saturating_sub(1);
    }

    pub const fn will_die(&self) -> bool {
        self.life == 0
    }
}

struct Food {}

impl Food {
    pub fn new() -> Self {
        Self {}
    }
}

fn simulation(
    mut commands: Commands,
    mut simulation: ResMut<SimulationState>,
    mut turn_count: ResMut<TurnCount>,
    sprites: Res<GameSprites>,
) {
    match &mut *simulation {
        SimulationState::Prepare {
            creature_count,
            food_count,
        } => {

            for i in 0..*creature_count {
                commands
                    .spawn(SpriteComponents {
                        material: sprites.creature,
                        transform: Transform::from_translation(Vec3::new(
                            i as f32 * 60.0,
                            0.0,
                            1.0,
                        )),
                        sprite: Sprite::new(Vec2::new(40.0, 40.0)),
                        ..Default::default()
                    })
                    .with(Creature::new());
            }

            for i in 0..*food_count {
                commands
                    .spawn(SpriteComponents {
                        material: sprites.food,
                        transform: Transform::from_translation(Vec3::new(
                            i as f32 * 50.0,
                            -100.0,
                            1.0,
                        )),
                        sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                        ..Default::default()
                    })
                    .with(Food::new());
            }

            *simulation = SimulationState::running();
            turn_count.0 = 0;
        }

        SimulationState::Running => {
            return;
        }
    }
}

fn eat_system(
    mut commands: Commands,
    mut creature_query: Query<(&mut Creature, &Transform, &Sprite)>,
    mut food_query: Query<(Entity, &Food, &Transform, &Sprite)>,
) {
    for (mut creature, creature_transform, sprite) in &mut creature_query.iter() {
        let creature_size = sprite.size;

        for (food_entity, _food, food_transform, sprite) in &mut food_query.iter() {
            let collision = collide(
                creature_transform.translation(),
                creature_size,
                food_transform.translation(),
                sprite.size,
            );

            if let Some(_collision) = collision {
                creature.eat_food();
                commands.despawn(food_entity);
            }
        }
    }
}

fn movement_system(time: Res<Time>, mut creature_query: Query<(&Creature, &mut Transform)>) {
    let delta_seconds = f32::min(0.2, time.delta_seconds);

    for (creature, mut transform) in &mut creature_query.iter() {
        transform.translate((creature.velocity * delta_seconds).extend(0.0));
    }
}

fn turn_system(
    mut commands: Commands,
    time: Res<Time>,
    simulation: Res<SimulationState>,
    mut turn_timer: ResMut<TurnTimer>,
    mut turn_count: ResMut<TurnCount>,
    mut creature_query: Query<(Entity, &mut Creature)>,
) {
    if !simulation.is_running() {
        return;
    }

    turn_timer.0.tick(time.delta_seconds);

    if !turn_timer.0.finished {
        return;
    }

    turn_count.0 += 1;

    for (creature_entity, mut creature) in &mut creature_query.iter() {
        creature.time_pass();

        if creature.will_die() {
            commands.despawn(creature_entity);
        } else {
            // TODO: duplicate creature
        }
    }
}

fn life_display_system(
    sprites: Res<GameSprites>,
    mut creature_query: Query<(&Creature, &mut SpriteComponents)>,
) {
    for (creature, mut sprite) in &mut creature_query.iter() {
        sprite.material = if creature.will_die() {
            sprites.creature
        } else {
            sprites.creature_filled
        };
    }
}
