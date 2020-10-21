mod bf;

use self::bf::{run as run_bf, Instruction};
use num_traits::Pow;
use rand::{thread_rng, Rng};
use rand::distributions::Standard;
use bevy::prelude::Vec2;

pub struct DNA {
    code: Vec<Instruction>,
}

impl DNA {
    pub fn generate() -> Self {
        let mut rng = thread_rng();

        Self {
            code: rng.sample_iter::<Standard, Instruction>().collect(),
        }
    }

    pub fn move_behaivor(&self) -> Vec2 {
        let output = run_bf(&self.code);
    }

    pub fn time_cost(&self) -> f32 {
        // TODO: relate this value with code size
        0.0
    }

    pub fn duplicate(&self) -> Self {
        // TODO: mutation
        Self {
            code: self.code.clone(),
        }
    }
}
