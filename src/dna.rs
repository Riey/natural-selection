mod bf;

use self::bf::{run as run_bf, Instruction};
use crate::utils::{convert_from_unit, convert_vec2_to_unit};
use bevy::prelude::Vec2;
use once_cell::sync::Lazy;
use crossbeam_channel::Receiver;
use rayon::prelude::*;
use rand::distributions::Standard;
use rand::{thread_rng, Rng};

static DNA_STORAGE: Lazy<Receiver<DNA>> = Lazy::new(|| {
    let (tx, rx) = crossbeam_channel::bounded(1024);

    std::thread::spawn(move || {
        std::iter::repeat_with(DNA::generate)
            .par_bridge()
            .for_each(move |dna| {
                tx.send(dna).unwrap();
            });
    });

    rx
});

#[derive(Clone)]
pub struct DNA {
    code: Vec<Instruction>,
}

impl DNA {
    pub fn generate() -> Self {
        let rng = thread_rng();

        Self {
            code: rng.sample_iter(Standard).take(2048).collect(),
        }
    }

    pub fn generate_prechecked() -> Self {
        DNA_STORAGE.recv().unwrap()
    }

    pub fn move_behaivor(&self, translation: Vec2) -> Result<Vec2, ()> {
        let (x, y) = convert_vec2_to_unit(translation);
        let output = run_bf(&self.code, &[x, y])?;
        let x = output.get(0).copied().map(convert_from_unit).unwrap_or(0.0);
        let y = output.get(1).copied().map(convert_from_unit).unwrap_or(0.0);

        Ok(Vec2::new(x, y))
    }

    pub fn time_cost(&self) -> f32 {
        // TODO: relate this value with code size
        0.5
    }

    pub fn mutate(&mut self) {
        let mut rng = thread_rng();

        for _ in 0..rng.gen_range(0, 5) {
            let idx = rng.gen_range(0, self.code.len());
            self.code[idx] = rng.gen();
        }
    }

    pub fn duplicate(&self) -> Self {
        let mut new_dna = self.clone();

        new_dna.mutate();

        new_dna
    }
}
