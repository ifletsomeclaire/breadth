use breadth::vector::movement::Movement;
use ultraviolet::{Vec3, f32x8};

fn main() {
    let mut movement = Movement::new(1);
    let my_vec = Vec3::new(1.0, 2.0, 3.0);
    for _ in 0..50 {
        let indy = movement.push(&my_vec, &my_vec, &my_vec);
    }
    movement.calculate(f32x8::splat(3.0));
    println!("{:#?}", movement);
}
