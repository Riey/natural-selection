use crate::dna::DNA;
use crate::utils::calculate_move_cost;

use bevy::math::{const_vec2, Vec2};
use bevy::prelude::Timer;

use std::time::Duration;

pub struct SimulationUi;

pub struct Wall;

pub struct Creature {
    life: f32,
    old: usize,
    generation: usize,
    activated: bool,
    velocity: Vec2,
    tick_timer: Timer,
    dna: DNA,
}

impl Creature {
    pub const INIT_X: usize = 40;
    pub const INIT_Y: usize = 40;
    pub const INIT_SIZE: Vec2 = const_vec2!([Self::INIT_X as f32, Self::INIT_Y as f32]);

    pub fn new() -> Self {
        Self {
            life: 0.0,
            old: 0,
            generation: 0,
            activated: false,
            velocity: Vec2::new(0.0, 0.0),
            tick_timer: Timer::new(Duration::from_millis(100), true),
            dna: DNA::generate_prechecked(),
        }
    }

    fn child(parent: &Self) -> Self {
        Self {
            life: 0.0,
            old: 0,
            generation: parent.generation + 1,
            activated: false,
            velocity: -parent.velocity,
            tick_timer: parent.tick_timer.clone(),
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

    pub fn time_pass(&mut self) {
        self.old += 1;
        self.life -= self.dna.time_cost();
    }

    pub fn will_die(&self) -> bool {
        !self.activated || self.life < self.dna.time_cost()
    }

    pub fn has_moved(&mut self, distance: f32) {
        if distance >= 0.0 {
            self.activated = true;
        }
        self.life -= calculate_move_cost(distance);
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn old(&self) -> usize {
        self.old
    }

    pub fn tick(&mut self, translation: Vec2, delta: Duration) -> Result<(), ()> {
        self.tick_timer.tick(delta);

        if self.tick_timer.finished() {
            self.velocity = self.dna.move_behaivor(translation)?;
        }

        Ok(())
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
