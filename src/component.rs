use bevy::math::{const_vec2, Vec2};

use crate::dna::DNA;
use bevy::core::Timer;

pub struct SimulationUi;

pub struct Wall;

pub struct Creature {
    life: f32,
    velocity: Vec2,
    move_timer: Timer,
    dna: DNA,
}

impl Creature {
    pub const INIT_X: usize = 40;
    pub const INIT_Y: usize = 40;
    pub const INIT_SIZE: Vec2 = const_vec2!([Self::INIT_X as f32, Self::INIT_Y as f32]);

    pub fn new() -> Self {
        Self {
            life: 0.0,
            velocity: Vec2::new(0.0, 0.0),
            move_timer: Timer::from_seconds(1.0, true),
            dna: DNA::generate(),
        }
    }

    fn child(parent: &Self) -> Self {
        Self {
            life: 0.0,
            velocity: -parent.velocity,
            move_timer: Timer::from_seconds(1.0, true),
            dna: parent.dna.duplicate(),
        }
    }

    pub fn try_eat_food(&mut self, food: &mut Food) -> bool {
        if food.try_ate() {
            self.life += 2.0;
            true
        } else {
            false
        }
    }

    pub fn crash_with_wall(&mut self) {
        self.velocity = -self.velocity;
    }

    pub fn time_pass(&mut self) {
        self.life -= self.dna.time_cost();
    }

    pub fn will_die(&self) -> bool {
        self.life < self.dna.time_cost()
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity * self.dna.move_speed();
    }

    pub fn move_timer(&mut self) -> &mut Timer {
        &mut self.move_timer
    }

    pub fn try_duplicate(&mut self) -> Option<Self> {
        if self.life > 1.0 + self.dna.time_cost() {
            self.life -= 1.0;
            Some(Self::child(self))
        } else {
            None
        }
    }
}

pub struct Food {
    is_ate: bool,
}

impl Food {
    pub const INIT_X: usize = 20;
    pub const INIT_Y: usize = 20;
    pub const INIT_SIZE: Vec2 = const_vec2!([Self::INIT_X as f32, Self::INIT_Y as f32]);

    pub fn new() -> Self {
        Self { is_ate: false }
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
