use super::*;
use rayon::iter::*;
use rayon::prelude::*;

// pub type Index = (usize, usize);

#[derive(Debug, Default, Copy, Clone)]
pub struct Index {
    pub vector: usize,
    pub scalar: usize,
}

#[derive(Debug, Default)]
pub struct Movement {
    pub transform: Vec<Mat4x8>,
    pub velocity: Vec<Vec3x8>,
    pub accel: Vec<Vec3x8>,
    pub next_index: Index,
    pub dead_indices: Vec<Index>,
}

impl Movement {
    pub fn new(size: usize) -> Self {
        let mut transform = Vec::with_capacity(size);
        transform.push(Mat4x8::default());
        let mut velocity = Vec::with_capacity(size);
        velocity.push(Vec3x8::default());
        let mut accel = Vec::with_capacity(size);
        accel.push(Vec3x8::default());

        Self {
            transform,
            velocity,
            accel,
            next_index: Index::default(),
            dead_indices: Vec::new(),
        }
    }
    pub fn push(&mut self, transform: &Mat4, velocity: &Vec3, accel: &Vec3) -> Index {
        match self.dead_indices.pop() {
            Some(index) => {
                clear_mat4_index(&mut self.transform[index.vector], index.scalar);
                write_mat4_index(&mut self.transform[index.vector], index.scalar, transform);
                clear_index(&mut self.velocity[index.vector], index.scalar);
                write_index(&mut self.velocity[index.vector], index.scalar, velocity);
                clear_index(&mut self.accel[index.vector], index.scalar);
                write_index(&mut self.accel[index.vector], index.scalar, accel);
                index
            }
            None => {
                let index = self.next_index;
                write_mat4_index(&mut self.transform[index.vector], index.scalar, transform);
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
                self.transform.push(Mat4x8::default());
                self.velocity.push(Vec3x8::default());
                self.accel.push(Vec3x8::default());
            }
            _ => {
                self.next_index.scalar += 1;
            }
        }
    }
    pub fn calculate(&mut self, timestep: f32x8) {
        self.transform
            .par_iter_mut()
            .zip(&mut self.velocity)
            .zip(&mut self.accel)
            .for_each(|((p, v), a)| {
                p.transform_vec3(*v * timestep);
            });
    }
    fn to_raw(&self) -> Vec<InstanceRaw> {
        let mut raw_mats = Vec::new();
        for wide_pos in self.transform.iter() {
            let c0: [f32x8; 4] = wide_pos[0].into();
            let c0x: [f32; 8] = c0[0].into();
            let c0y: [f32; 8] = c0[1].into();
            let c0z: [f32; 8] = c0[2].into();
            let c0w: [f32; 8] = c0[3].into();

            let c1: [f32x8; 4] = wide_pos[1].into();
            let c1x: [f32; 8] = c1[0].into();
            let c1y: [f32; 8] = c1[1].into();
            let c1z: [f32; 8] = c1[2].into();
            let c1w: [f32; 8] = c1[3].into();

            let c2: [f32x8; 4] = wide_pos[2].into();
            let c2x: [f32; 8] = c2[0].into();
            let c2y: [f32; 8] = c2[1].into();
            let c2z: [f32; 8] = c2[2].into();
            let c2w: [f32; 8] = c2[3].into();

            let c3: [f32x8; 4] = wide_pos[3].into();
            let c3x: [f32; 8] = c3[0].into();
            let c3y: [f32; 8] = c3[1].into();
            let c3z: [f32; 8] = c3[2].into();
            let c3w: [f32; 8] = c3[3].into();

            for i in 0..8 {
                raw_mats.push(InstanceRaw {
                    model: Mat4::new(
                        Vec4::from((c0x[i], c0y[i], c0z[i], c0w[i])),
                        Vec4::from((c1x[i], c1y[i], c1z[i], c1w[i])),
                        Vec4::from((c2x[i], c2y[i], c2z[i], c2w[i])),
                        Vec4::from((c3x[i], c3y[i], c3z[i], c3w[i])),
                    ),
                });
            }
        }
        raw_mats
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstanceRaw {
    model: Mat4,
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
fn clear_mat4_index(wide_vec: &mut Mat4x8, index: usize) {
    let mut c0x: [f32; 8] = [1.0; 8];
    c0x[index] = 0.0;
    let mut c0y: [f32; 8] = [1.0; 8];
    c0y[index] = 0.0;
    let mut c0z: [f32; 8] = [1.0; 8];
    c0z[index] = 0.0;
    let mut c0w: [f32; 8] = [1.0; 8];
    c0w[index] = 0.0;
    let mut c1x: [f32; 8] = [1.0; 8];
    c1x[index] = 0.0;
    let mut c1y: [f32; 8] = [1.0; 8];
    c1y[index] = 0.0;
    let mut c1z: [f32; 8] = [1.0; 8];
    c1z[index] = 0.0;
    let mut c1w: [f32; 8] = [1.0; 8];
    c1w[index] = 0.0;
    let mut c2x: [f32; 8] = [1.0; 8];
    c2x[index] = 0.0;
    let mut c2y: [f32; 8] = [1.0; 8];
    c2y[index] = 0.0;
    let mut c2z: [f32; 8] = [1.0; 8];
    c2z[index] = 0.0;
    let mut c2w: [f32; 8] = [1.0; 8];
    c2w[index] = 0.0;
    let mut c3x: [f32; 8] = [1.0; 8];
    c3x[index] = 0.0;
    let mut c3y: [f32; 8] = [1.0; 8];
    c3y[index] = 0.0;
    let mut c3z: [f32; 8] = [1.0; 8];
    c3z[index] = 0.0;
    let mut c3w: [f32; 8] = [1.0; 8];
    c3w[index] = 0.0;

    let c0 = Vec4x8::from((
        f32x8::from(c0x),
        f32x8::from(c0y),
        f32x8::from(c0z),
        f32x8::from(c0w),
    ));
    let c1 = Vec4x8::from((
        f32x8::from(c1x),
        f32x8::from(c1y),
        f32x8::from(c1z),
        f32x8::from(c1w),
    ));
    let c2 = Vec4x8::from((
        f32x8::from(c2x),
        f32x8::from(c2y),
        f32x8::from(c2z),
        f32x8::from(c2w),
    ));
    let c3 = Vec4x8::from((
        f32x8::from(c3x),
        f32x8::from(c3y),
        f32x8::from(c3z),
        f32x8::from(c3w),
    ));

    wide_vec.cols[0] *= c0;
    wide_vec.cols[1] *= c1;
    wide_vec.cols[2] *= c2;
    wide_vec.cols[3] *= c3;
}

fn write_mat4_index(wide_vec: &mut Mat4x8, index: usize, new: &Mat4) {
    let mut c0x: [f32; 8] = [1.0; 8];
    c0x[index] = new.cols[0].x;
    let mut c0y: [f32; 8] = [1.0; 8];
    c0y[index] = new.cols[0].y;
    let mut c0z: [f32; 8] = [1.0; 8];
    c0z[index] = new.cols[0].z;
    let mut c0w: [f32; 8] = [1.0; 8];
    c0w[index] = new.cols[0].w;
    let mut c1x: [f32; 8] = [1.0; 8];
    c1x[index] = new.cols[1].x;
    let mut c1y: [f32; 8] = [1.0; 8];
    c1y[index] = new.cols[1].y;
    let mut c1z: [f32; 8] = [1.0; 8];
    c1z[index] = new.cols[1].z;
    let mut c1w: [f32; 8] = [1.0; 8];
    c1w[index] = new.cols[1].w;
    let mut c2x: [f32; 8] = [1.0; 8];
    c2x[index] = new.cols[2].x;
    let mut c2y: [f32; 8] = [1.0; 8];
    c2y[index] = new.cols[2].y;
    let mut c2z: [f32; 8] = [1.0; 8];
    c2z[index] = new.cols[2].z;
    let mut c2w: [f32; 8] = [1.0; 8];
    c2w[index] = new.cols[2].w;
    let mut c3x: [f32; 8] = [1.0; 8];
    c3x[index] = new.cols[3].x;
    let mut c3y: [f32; 8] = [1.0; 8];
    c3y[index] = new.cols[3].y;
    let mut c3z: [f32; 8] = [1.0; 8];
    c3z[index] = new.cols[3].z;
    let mut c3w: [f32; 8] = [1.0; 8];
    c3w[index] = new.cols[3].w;

    let c0 = Vec4x8::from((
        f32x8::from(c0x),
        f32x8::from(c0y),
        f32x8::from(c0z),
        f32x8::from(c0w),
    ));
    let c1 = Vec4x8::from((
        f32x8::from(c1x),
        f32x8::from(c1y),
        f32x8::from(c1z),
        f32x8::from(c1w),
    ));
    let c2 = Vec4x8::from((
        f32x8::from(c2x),
        f32x8::from(c2y),
        f32x8::from(c2z),
        f32x8::from(c2w),
    ));
    let c3 = Vec4x8::from((
        f32x8::from(c3x),
        f32x8::from(c3y),
        f32x8::from(c3z),
        f32x8::from(c3w),
    ));

    wide_vec.cols[0] += c0;
    wide_vec.cols[1] += c1;
    wide_vec.cols[2] += c2;
    wide_vec.cols[3] += c3;
}
