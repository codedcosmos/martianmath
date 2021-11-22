use std::{fmt, ops};
use impl_ops::*;
use crate::vector2f::Vector2f;
use crate::vector4f::Vector4f;

#[derive(Copy, Clone, Debug)]
pub struct Vector3f {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

// Custom
impl Vector3f {
	pub fn new() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0
		}
	}

	pub fn from(x: f32, y: f32, z: f32) -> Self {
		Self {
			x,
			y,
			z
		}
	}

	pub fn max(&self) -> f32 {
		self.x.max(self.y).max(self.z)
	}

	pub fn min(&self) -> f32 {
		self.x.min(self.y).min(self.z)
	}

	pub fn absolute(&self) -> Vector3f {
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
			z: self.z.abs()
		}
	}

	pub fn to_cartesian(&self) -> Vector2f {
		Vector2f::from(self.x / self.z, self.y / self.z)
	}

	pub fn to_homogeneous(&self) -> Vector4f {
		Vector4f::from(self.x, self.y, self.z, 1.0)
	}

	pub fn dot(&self, b: Self) -> f32 {
		self.x*b.x + self.y*b.y + self.z*b.z
	}

	pub fn cross(&self, b: Self) -> Self {
		Self {
			x: self.y * b.z - self.z * b.y,
			y: self.z * b.x - self.x * b.z,
			z: self.x * b.y - self.y - b.x
		}
	}

	pub fn length(&self) -> f32 {
		(self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
	}

	pub fn normalise(&self) -> Self {
		let len = self.length();

		Self {
			x: self.x / len,
			y: self.y / len,
			z: self.z / len
		}
	}
}

// Display
impl fmt::Display for Vector3f {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Use `self.number` to refer to each positional data point.
		write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
	}
}

// Add subtract
impl_op_ex!(+ |a: Vector3f, b: Vector3f| -> Vector3f { Vector3f { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z } });
impl_op_ex!(- |a: Vector3f, b: Vector3f| -> Vector3f { Vector3f { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z } });

// Scalar operations
impl_op_ex!(+ |a: Vector3f, b: f32| -> Vector3f { Vector3f { x: a.x + b, y: a.y + b, z: a.z + b } });
impl_op_ex!(+ |a: f32, b: Vector3f| -> Vector3f { Vector3f { x: a + b.x, y: a + b.y, z: a + b.z } });
impl_op_ex!(- |a: Vector3f, b: f32| -> Vector3f { Vector3f { x: a.x - b, y: a.y - b, z: a.z - b } });
impl_op_ex!(- |a: f32, b: Vector3f| -> Vector3f { Vector3f { x: a - b.x, y: a - b.y, z: a - b.z } });
impl_op_ex!(* |a: Vector3f, b: f32| -> Vector3f { Vector3f { x: a.x * b, y: a.y * b, z: a.z * b } });
impl_op_ex!(* |a: f32, b: Vector3f| -> Vector3f { Vector3f { x: a * b.x, y: a * b.y, z: a * b.z } });
impl_op_ex!(/ |a: Vector3f, b: f32| -> Vector3f { Vector3f { x: a.x / b, y: a.y / b, z: a.z / b } });
impl_op_ex!(/ |a: f32, b: Vector3f| -> Vector3f { Vector3f { x: a / b.x, y: a / b.y, z: a / b.z } });

// Unary
impl_op_ex!(- |a: Vector3f| -> Vector3f { Vector3f { x: -a.x, y: -a.y, z: -a.z } });