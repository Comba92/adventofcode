pub type Vector = (i32, i32);

pub const LEFT: Vector = (-1, 0);
pub const RIGHT: Vector = (1, 0);
pub const UP: Vector = (0, -1);
pub const DOWN: Vector = (0, 1);

pub const DIRECTIONS: [Vector; 4] = [
  LEFT, UP, RIGHT, DOWN
];

pub fn vec_add(a: Vector, b: Vector) -> Vector {
  (a.0 + b.0, a.1 + b.1)
}