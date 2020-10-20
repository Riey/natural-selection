use bevy::math::const_vec2;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use grid::Grid;
use rand::{seq::IteratorRandom, thread_rng, Rng};

const BACK_COLOR: Color = Color::BLACK;
const GRID_SIZE: (usize, usize) = (800, 300);
const GRID_BOUND: Vec2 = const_vec2!([GRID_SIZE.0 as f32, GRID_SIZE.1 as f32]);

fn main() {
    App::build()
        .add_default_plugins()
        .add_resource(ClearColor(BACK_COLOR))
        .add_resource(SimulationState::prepare(10, 10, 50))
        .add_resource(TurnTimer(Timer::from_seconds(10.0, true)))
        .add_resource(TurnCount(0))
        .add_startup_system(setup.system())
        .add_system(prepare_simulation.system())
        .add_system(collision_system.system())
        .add_system(movement_system.system())
        .add_system(turn_system.system())
        .add_system(life_display_system.system())
        .add_system(random_move_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    let wall_material = materials.add(Color::WHITE.into());
    let thickness = 10.0;
    let bound = GRID_BOUND + Vec2::splat(thickness);

    commands
        .spawn(SpriteComponents {
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(-bound.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(thickness, bound.y() + thickness)),
            ..Default::default()
        })
        .with(Wall)
        .spawn(SpriteComponents {
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(bound.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(thickness, bound.y() + thickness)),
            ..Default::default()
        })
        .with(Wall)
        .spawn(SpriteComponents {
            material: wall_material,
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

struct Wall;

struct GameSprites {
    creature: Handle<ColorMaterial>,
    creature_filled: Handle<ColorMaterial>,
    food: Handle<ColorMaterial>,
}

enum SimulationState {
    Prepare {
        creature_count: usize,
        food_count: usize,
        daily_food_count: usize,
    },
    Running {
        daily_food_count: usize,
    },
}

impl SimulationState {
    pub fn prepare(creature_count: usize, food_count: usize, daily_food_count: usize) -> Self {
        SimulationState::Prepare {
            creature_count,
            food_count,
            daily_food_count,
        }
    }

    pub fn running(daily_food_count: usize) -> Self {
        SimulationState::Running { daily_food_count }
    }
}

struct TurnTimer(Timer);
struct TurnCount(usize);

struct Creature {
    life: usize,
    velocity: Vec2,
    move_timer: Timer,
}

impl Creature {
    pub const INIT_X: usize = 40;
    pub const INIT_Y: usize = 40;
    pub const INIT_SIZE: Vec2 = const_vec2!([Self::INIT_X as f32, Self::INIT_Y as f32]);

    pub fn new() -> Self {
        Self {
            life: 0,
            velocity: Vec2::new(0.0, 0.0),
            move_timer: Timer::from_seconds(1.0, true),
        }
    }

    pub fn try_eat_food(&mut self, food: &mut Food) -> bool {
        if food.try_ate() {
            self.life += 1;
            true
        } else {
            false
        }
    }

    pub fn crash_with_wall(&mut self) {
        self.velocity = -self.velocity;
    }

    pub fn time_pass(&mut self) {
        self.life = self.life.saturating_sub(1);
    }

    pub const fn will_die(&self) -> bool {
        self.life == 0
    }

    pub fn velocity_mut(&mut self) -> &mut Vec2 {
        &mut self.velocity
    }

    pub fn move_timer(&mut self) -> &mut Timer {
        &mut self.move_timer
    }

    pub fn try_duplicate(&mut self) -> bool {
        if self.life >= 2 {
            self.life -= 1;
            true
        } else {
            false
        }
    }
}

struct Food {
    is_ate: bool,
}

impl Food {
    pub const INIT_X: usize = 20;
    pub const INIT_Y: usize = 20;
    pub const INIT_SIZE: Vec2 = const_vec2!([Self::INIT_X as f32, Self::INIT_Y as f32]);

    pub fn new() -> Self {
        Self {
            is_ate: false,
        }
    }

    pub fn try_ate(&mut self) -> bool {
        if !self.is_ate {
            self.is_ate = true;
            true
        } else {
            false
        }
    }
}

fn prepare_simulation(
    mut commands: Commands,
    mut simulation: ResMut<SimulationState>,
    mut turn_count: ResMut<TurnCount>,
    sprites: Res<GameSprites>,
) {
    if let SimulationState::Prepare {
        creature_count,
        food_count,
        daily_food_count,
    } = &mut *simulation
    {
        for i in 0..*creature_count {
            commands
                .spawn(SpriteComponents {
                    material: sprites.creature,
                    transform: Transform::from_translation(Vec3::new(i as f32 * 60.0, 0.0, 1.0)),
                    sprite: Sprite::new(Creature::INIT_SIZE),
                    ..Default::default()
                })
                .with(Creature::new());
        }

        for i in 0..*food_count {
            commands
                .spawn(SpriteComponents {
                    material: sprites.food,
                    transform: Transform::from_translation(Vec3::new(i as f32 * 50.0, -100.0, 1.0)),
                    sprite: Sprite::new(Food::INIT_SIZE),
                    ..Default::default()
                })
                .with(Food::new());
        }

        *simulation = SimulationState::running(*daily_food_count);
        turn_count.0 = 0;
    }
}

fn collision_system(
    mut commands: Commands,
    mut creature_query: Query<(&mut Creature, &Transform, &Sprite)>,
    mut food_query: Query<(Entity, &mut Food, &Transform, &Sprite)>,
    mut wall_query: Query<(&Wall, &Transform, &Sprite)>,
) {
    for (mut creature, creature_transform, sprite) in &mut creature_query.iter() {
        let creature_size = sprite.size;

        for (food_entity, mut food, food_transform, sprite) in &mut food_query.iter() {
            let collision = collide(
                creature_transform.translation(),
                creature_size,
                food_transform.translation(),
                sprite.size,
            );

            if let Some(_collision) = collision {
                // eat
                if creature.try_eat_food(&mut food) {
                    println!("yummy");
                    commands.despawn_recursive(food_entity);
                }
            }
        }

        for (_wall, wall_transform, sprite) in &mut wall_query.iter() {
            let collision = collide(
                creature_transform.translation(),
                creature_size,
                wall_transform.translation(),
                sprite.size,
            );

            if let Some(_collision) = collision {
                // crash
                creature.crash_with_wall();
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

fn calculate_random_objects(
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

    grid.iter_mut()
        .enumerate()
        .filter_map(|(i, v)| if *v { None } else { Some(i) })
        .choose_multiple(&mut rng, count)
        .into_iter()
        .map(move |idx| {
            let y = idx / grid.cols();
            let x = idx - y * grid.cols();
            Transform::from_translation(Vec3::new(
                (x * object_width) as f32 - (GRID_SIZE.0 / 2) as f32,
                (y * object_height) as f32 - (GRID_SIZE.1 / 2) as f32,
                0.0,
            ))
        })
}

fn turn_system(
    mut commands: Commands,
    time: Res<Time>,
    simulation: Res<SimulationState>,
    sprites: Res<GameSprites>,
    mut turn_timer: ResMut<TurnTimer>,
    mut turn_count: ResMut<TurnCount>,
    mut creature_query: Query<(Entity, &mut Creature, &Transform)>,
    mut food_query: Query<(&Food, &Transform)>,
) {
    if let SimulationState::Running { daily_food_count } = &*simulation {
        turn_timer.0.tick(time.delta_seconds);

        if !turn_timer.0.finished {
            return;
        }

        turn_count.0 += 1;

        // Process creature
        for (creature_entity, mut creature, transform) in &mut creature_query.iter() {
            if creature.will_die() {
                commands.despawn(creature_entity);
            } else {
                if creature.try_duplicate() {
                    commands
                        .spawn(SpriteComponents {
                            material: sprites.creature,
                            sprite: Sprite::new(Creature::INIT_SIZE),
                            transform: transform.clone(),
                            ..Default::default()
                        })
                        .with(Creature::new());
                }
            }

            creature.time_pass();
        }

        // Spawn foods

        let mut food_iter = food_query.iter();

        for transform in calculate_random_objects(
            Food::INIT_X,
            Food::INIT_Y,
            *daily_food_count,
            food_iter
                .iter()
                .map(|(_food, transform)| transform.translation()),
        ) {
            commands
                .spawn(SpriteComponents {
                    material: sprites.food,
                    sprite: Sprite::new(Food::INIT_SIZE),
                    transform,
                    ..Default::default()
                })
                .with(Food::new());
        }
    }
}

fn life_display_system(
    sprites: Res<GameSprites>,
    mut creature_query: Query<(&Creature, &mut Handle<ColorMaterial>)>,
) {
    for (creature, mut sprite) in &mut creature_query.iter() {
        *sprite = if creature.will_die() {
            sprites.creature
        } else {
            sprites.creature_filled
        };
    }
}

fn random_move_system(time: Res<Time>, mut creature_query: Query<(&mut Creature,)>) {
    const MAX_SPEED: f32 = 150.0 / 2.0;

    let mut rng = thread_rng();

    for (mut creature,) in &mut creature_query.iter() {
        creature.move_timer().tick(time.delta_seconds);

        if !creature.move_timer().finished {
            continue;
        }

        let x = rng.gen_range(-MAX_SPEED, MAX_SPEED);
        let y = rng.gen_range(-MAX_SPEED, MAX_SPEED);
        *creature.velocity_mut().x_mut() = x;
        *creature.velocity_mut().y_mut() = y;
    }
}
