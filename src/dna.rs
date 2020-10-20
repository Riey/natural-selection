use num_traits::Pow;
use rand::{Rng, thread_rng};

pub struct DNA {
    move_speed: f32,
}

impl DNA {
    pub fn generate() -> Self {
        let mut rng = thread_rng();

        Self {
            move_speed: rng.gen_range(30.0, 75.0),
        }
    }

    pub fn move_speed(&self) -> f32 {
        self.move_speed
    }

    pub fn time_cost(&self) -> f32 {
        1.0 + self.move_speed().pow(1.2) / 80.0
    }
}
