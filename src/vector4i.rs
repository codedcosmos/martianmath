use std::{fmt, ops};
use impl_ops::*;
use crate::vector3i::Vector3i;

#[derive(Copy, Clone, Debug)]
pub struct Vector4i {
	pub x: i32,
	pub y: i32,
	pub z: i32,
	pub w: i32,
}

// Custom
impl Vector4i {
	pub fn new() -> Self {
		Self {
			x: 0,
			y: 0,
			z: 0,
			w: 0
		}
	}

	pub fn from(x: i32, y: i32, z: i32, w: i32) -> Self {
		Self {
			x,
			y,
			z,
			w
		}
	}

	pub fn max(&self) -> i32 {
		self.x.max(self.y).max(self.z).max(self.w)
	}

	pub fn min(&self) -> i32 {
		self.x.min(self.y).min(self.z).min(self.w)
	}

	pub fn absolute(&self) -> Vector4i {
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
			z: self.z.abs(),
			w: self.w.abs()
		}
	}

	pub fn to_cartesian(&self) -> Vector3i {
		Vector3i::from(self.x / self.w, self.y / self.w, self.z / self.w)
	}
}

// Display
impl fmt::Display for Vector4i {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Use `self.number` to refer to each positional data point.
		write!(f, "<{}, {}, {}, {}>", self.x, self.y, self.z, self.w)
	}
}

// Add subtract
impl_op_ex!(+ |a: Vector4i, b: Vector4i| -> Vector4i { Vector4i { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z, w: a.w + b.w } });
impl_op_ex!(- |a: Vector4i, b: Vector4i| -> Vector4i { Vector4i { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z, w: a.w - b.w } });

// Scalar operations
impl_op_ex!(+ |a: Vector4i, b: i32| -> Vector4i { Vector4i { x: a.x + b, y: a.y + b, z: a.z + b, w: a.w + b } });
impl_op_ex!(+ |a: i32, b: Vector4i| -> Vector4i { Vector4i { x: a + b.x, y: a + b.y, z: a + b.z, w: a + b.w } });
impl_op_ex!(- |a: Vector4i, b: i32| -> Vector4i { Vector4i { x: a.x - b, y: a.y - b, z: a.z - b, w: a.w - b } });
impl_op_ex!(- |a: i32, b: Vector4i| -> Vector4i { Vector4i { x: a - b.x, y: a - b.y, z: a - b.z, w: a - b.w } });
impl_op_ex!(* |a: Vector4i, b: i32| -> Vector4i { Vector4i { x: a.x * b, y: a.y * b, z: a.z * b, w: a.w * b } });
impl_op_ex!(* |a: i32, b: Vector4i| -> Vector4i { Vector4i { x: a * b.x, y: a * b.y, z: a * b.z, w: a * b.w } });
impl_op_ex!(/ |a: Vector4i, b: i32| -> Vector4i { Vector4i { x: a.x / b, y: a.y / b, z: a.z / b, w: a.w / b } });
impl_op_ex!(/ |a: i32, b: Vector4i| -> Vector4i { Vector4i { x: a / b.x, y: a / b.y, z: a / b.z, w: a / b.w } });

// Unary
impl_op_ex!(- |a: Vector4i| -> Vector4i { Vector4i { x: -a.x, y: -a.y, z: -a.z, w: -a.w } });