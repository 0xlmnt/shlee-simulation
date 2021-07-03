use nalgebra as na;
use rand::{Rng, RngCore};

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng)
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    food: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            animals: (0..40).map(|_| {
                Animal::random(rng)
            }).collect(),
            food: (0..40).map(|_| {
                Food::random(rng)
            }).collect(),
        }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn food(&self) -> &[Food] {
        &self.food
    }
}

#[derive(Debug)]
pub struct Animal {
    pos: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            pos: rng.gen(),
            rotation: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.pos
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}

#[derive(Debug)]
pub struct Food {
    pos: na::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            pos: rng.gen()
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.pos
    }
}


