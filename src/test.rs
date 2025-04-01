extern crate nannou;
use nannou::prelude::*;

fn main() {
    let mut ij: Vec2 = Vec2::new(4.0, 1.0);
    ij.normalize();
    println!("{}", ij[0]);
}
