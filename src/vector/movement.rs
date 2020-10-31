use super::*;
use rayon::prelude::*;
use rayon::iter::*;

// pub type Index = (usize, usize);

#[derive(Debug, Default, Copy, Clone)]
pub struct Index {
    pub vector: usize,
    pub scalar: usize,
}

#[derive(Debug, Default)]
pub struct Movement {
    pub position: Vec<Vec3x8>,
    pub velocity: Vec<Vec3x8>,
    pub accel: Vec<Vec3x8>,
    pub next_index: Index,
    pub dead_indices: Vec<Index>,
}

impl Movement {
    pub fn new(size: usize) -> Self {
        let mut position = Vec::with_capacity(size);
        position.push(Vec3x8::default());
        let mut velocity = Vec::with_capacity(size);
        velocity.push(Vec3x8::default());
        let mut accel = Vec::with_capacity(size);
        accel.push(Vec3x8::default());

        Self {
            position,
            velocity,
            accel,
            next_index: Index::default(),
            dead_indices: Vec::new(),
        }
    }
    pub fn push(&mut self, position: &Vec3, velocity: &Vec3, accel: &Vec3) -> Index {
        match self.dead_indices.pop() {
            Some(index) => {
                clear_index(&mut self.position[index.vector], index.scalar);
                write_index(&mut self.position[index.vector], index.scalar, position);
                clear_index(&mut self.velocity[index.vector], index.scalar);
                write_index(&mut self.velocity[index.vector], index.scalar, velocity);
                clear_index(&mut self.accel[index.vector], index.scalar);
                write_index(&mut self.accel[index.vector], index.scalar, accel);
                index
            }
            None => {
                let index = self.next_index;
                write_index(&mut self.position[index.vector], index.scalar, position);
                write_index(&mut self.velocity[index.vector], index.scalar, velocity);
                write_index(&mut self.accel[index.vector], index.scalar, accel);
                self.increment();
                index
            }
        }
    }
    fn increment(&mut self) {
        match self.next_index.scalar {
            7 => {
                self.next_index.vector += 1;
                self.next_index.scalar = 0;
                self.position.push(Vec3x8::default());
                self.velocity.push(Vec3x8::default());
                self.accel.push(Vec3x8::default());
            }
            _ => {
                self.next_index.scalar += 1;
            }
        }
    }
    pub fn calculate(&mut self, timestep: f32x8) {
        self.position.par_iter_mut().zip(&mut self.velocity).zip(&mut self.accel).for_each(|((p, v), a)|{
            *p += *v * timestep;
        });
    }
}

fn clear_index(wide_vec: &mut Vec3x8, index: usize) {
    let mut x: [f32; 8] = [1.0; 8];
    x[index] = 0.0;
    let mut y: [f32; 8] = [1.0; 8];
    y[index] = 0.0;
    let mut z: [f32; 8] = [1.0; 8];
    z[index] = 0.0;
    *wide_vec *= Vec3x8::new(f32x8::from(x), f32x8::from(y), f32x8::from(z));
}

fn write_index(wide_vec: &mut Vec3x8, index: usize, new: &Vec3) {
    let mut x: [f32; 8] = [0.0; 8];
    x[index] = new.x;
    let mut y: [f32; 8] = [0.0; 8];
    y[index] = new.y;
    let mut z: [f32; 8] = [0.0; 8];
    z[index] = new.z;
    *wide_vec += Vec3x8::new(f32x8::from(x), f32x8::from(y), f32x8::from(z));
}

