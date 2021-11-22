use std::{fmt, ops};
use impl_ops::*;
use crate::vector2i::Vector2i;
use crate::vector4i::Vector4i;

#[derive(Copy, Clone, Debug)]
pub struct Vector3i {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

// Custom
impl Vector3i {
	pub fn new() -> Self {
		Self {
			x: 0,
			y: 0,
			z: 0
		}
	}

	pub fn from(x: i32, y: i32, z: i32) -> Self {
		Self {
			x,
			y,
			z
		}
	}

	pub fn max(&self) -> i32 {
		self.x.max(self.y).max(self.z)
	}

	pub fn min(&self) -> i32 {
		self.x.min(self.y).min(self.z)
	}

	pub fn absolute(&self) -> Vector3i {
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
			z: self.z.abs()
		}
	}

	pub fn to_cartesian(&self) -> Vector2i {
		Vector2i::from(self.x / self.z, self.y / self.z)
	}

	pub fn to_homogeneous(&self) -> Vector4i {
		Vector4i::from(self.x, self.y, self.z, 1)
	}
}

// Display
impl fmt::Display for Vector3i {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Use `self.number` to refer to each positional data point.
		write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
	}
}

// Add subtract
impl_op_ex!(+ |a: Vector3i, b: Vector3i| -> Vector3i { Vector3i { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z } });
impl_op_ex!(- |a: Vector3i, b: Vector3i| -> Vector3i { Vector3i { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z } });

// Scalar operations
impl_op_ex!(+ |a: Vector3i, b: i32| -> Vector3i { Vector3i { x: a.x + b, y: a.y + b, z: a.z + b } });
impl_op_ex!(+ |a: i32, b: Vector3i| -> Vector3i { Vector3i { x: a + b.x, y: a + b.y, z: a + b.z } });
impl_op_ex!(- |a: Vector3i, b: i32| -> Vector3i { Vector3i { x: a.x - b, y: a.y - b, z: a.z - b } });
impl_op_ex!(- |a: i32, b: Vector3i| -> Vector3i { Vector3i { x: a - b.x, y: a - b.y, z: a - b.z } });
impl_op_ex!(* |a: Vector3i, b: i32| -> Vector3i { Vector3i { x: a.x * b, y: a.y * b, z: a.z * b } });
impl_op_ex!(* |a: i32, b: Vector3i| -> Vector3i { Vector3i { x: a * b.x, y: a * b.y, z: a * b.z } });
impl_op_ex!(/ |a: Vector3i, b: i32| -> Vector3i { Vector3i { x: a.x / b, y: a.y / b, z: a.z / b } });
impl_op_ex!(/ |a: i32, b: Vector3i| -> Vector3i { Vector3i { x: a / b.x, y: a / b.y, z: a / b.z } });

// Unary
impl_op_ex!(- |a: Vector3i| -> Vector3i { Vector3i { x: -a.x, y: -a.y, z: -a.z } });