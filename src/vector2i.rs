use std::{fmt, ops};
use impl_ops::*;
use crate::vector3i::Vector3i;

#[derive(Copy, Clone, Debug)]
pub struct Vector2i {
	pub x: i32,
	pub y: i32,
}

// Custom
impl Vector2i {
	pub fn new() -> Self {
		Self {
			x: 0,
			y: 0
		}
	}

	pub fn from(x: i32, y: i32) -> Self {
		Self {
			x,
			y
		}
	}

	pub fn max(&self) -> i32 {
		self.x.max(self.y)
	}

	pub fn min(&self) -> i32 {
		self.x.min(self.y)
	}

	pub fn absolute(&self) -> Vector2i {
		Self {
			x: self.x.abs(),
			y: self.y.abs()
		}
	}

	pub fn to_homogeneous(&self) -> Vector3i {
		Vector3i::from(self.x, self.y, 1)
	}
}

// Display
impl fmt::Display for Vector2i {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Use `self.number` to refer to each positional data point.
		write!(f, "<{}, {}>", self.x, self.y)
	}
}

// Add subtract
impl_op_ex!(+ |a: Vector2i, b: Vector2i| -> Vector2i { Vector2i { x: a.x + b.x, y: a.y + b.y } });
impl_op_ex!(- |a: Vector2i, b: Vector2i| -> Vector2i { Vector2i { x: a.x - b.x, y: a.y - b.y } });

// Scalar operations
impl_op_ex!(+ |a: Vector2i, b: i32| -> Vector2i { Vector2i { x: a.x + b, y: a.y + b } });
impl_op_ex!(+ |a: i32, b: Vector2i| -> Vector2i { Vector2i { x: a + b.x, y: a + b.y } });
impl_op_ex!(- |a: Vector2i, b: i32| -> Vector2i { Vector2i { x: a.x - b, y: a.y - b } });
impl_op_ex!(- |a: i32, b: Vector2i| -> Vector2i { Vector2i { x: a - b.x, y: a - b.y } });
impl_op_ex!(* |a: Vector2i, b: i32| -> Vector2i { Vector2i { x: a.x * b, y: a.y * b } });
impl_op_ex!(* |a: i32, b: Vector2i| -> Vector2i { Vector2i { x: a * b.x, y: a * b.y } });
impl_op_ex!(/ |a: Vector2i, b: i32| -> Vector2i { Vector2i { x: a.x / b, y: a.y / b } });
impl_op_ex!(/ |a: i32, b: Vector2i| -> Vector2i { Vector2i { x: a / b.x, y: a / b.y } });

// Unary
impl_op_ex!(- |a: Vector2i| -> Vector2i { Vector2i { x: -a.x, y: -a.y} });