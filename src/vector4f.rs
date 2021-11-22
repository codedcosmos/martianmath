use std::{fmt, ops};
use impl_ops::*;
use crate::vector3f::Vector3f;

#[derive(Copy, Clone, Debug)]
pub struct Vector4f {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

// Custom
impl Vector4f {
	pub fn new() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0,
			w: 0.0
		}
	}

	pub fn from(x: f32, y: f32, z: f32, w: f32) -> Self {
		Self {
			x,
			y,
			z,
			w
		}
	}

	pub fn max(&self) -> f32 {
		self.x.max(self.y).max(self.z).max(self.w)
	}

	pub fn min(&self) -> f32 {
		self.x.min(self.y).min(self.z).min(self.w)
	}

	pub fn absolute(&self) -> Vector4f {
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
			z: self.z.abs(),
			w: self.w.abs()
		}
	}

	pub fn to_cartesian(&self) -> Vector3f {
		Vector3f::from(self.x / self.w, self.y / self.w, self.z / self.w)
	}

	pub fn dot(&self, b: Self) -> f32 {
		self.x*b.x + self.y*b.y + self.z*b.z+ self.w*b.w
	}

	pub fn length(&self) -> f32 {
		(self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
	}

	pub fn normalise(&self) -> Self {
		let len = self.length();

		Self {
			x: self.x / len,
			y: self.y / len,
			z: self.z / len,
			w: self.w / len
		}
	}
}

// Display
impl fmt::Display for Vector4f {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Use `self.number` to refer to each positional data point.
		write!(f, "<{}, {}, {}, {}>", self.x, self.y, self.z, self.w)
	}
}

// Add subtract
impl_op_ex!(+ |a: Vector4f, b: Vector4f| -> Vector4f { Vector4f { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z, w: a.w + b.w } });
impl_op_ex!(- |a: Vector4f, b: Vector4f| -> Vector4f { Vector4f { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z, w: a.w - b.w } });

// Scalar operations
impl_op_ex!(+ |a: Vector4f, b: f32| -> Vector4f { Vector4f { x: a.x + b, y: a.y + b, z: a.z + b, w: a.w + b } });
impl_op_ex!(+ |a: f32, b: Vector4f| -> Vector4f { Vector4f { x: a + b.x, y: a + b.y, z: a + b.z, w: a + b.w } });
impl_op_ex!(- |a: Vector4f, b: f32| -> Vector4f { Vector4f { x: a.x - b, y: a.y - b, z: a.z - b, w: a.w - b } });
impl_op_ex!(- |a: f32, b: Vector4f| -> Vector4f { Vector4f { x: a - b.x, y: a - b.y, z: a - b.z, w: a - b.w } });
impl_op_ex!(* |a: Vector4f, b: f32| -> Vector4f { Vector4f { x: a.x * b, y: a.y * b, z: a.z * b, w: a.w * b } });
impl_op_ex!(* |a: f32, b: Vector4f| -> Vector4f { Vector4f { x: a * b.x, y: a * b.y, z: a * b.z, w: a * b.w } });
impl_op_ex!(/ |a: Vector4f, b: f32| -> Vector4f { Vector4f { x: a.x / b, y: a.y / b, z: a.z / b, w: a.w / b } });
impl_op_ex!(/ |a: f32, b: Vector4f| -> Vector4f { Vector4f { x: a / b.x, y: a / b.y, z: a / b.z, w: a / b.w } });

// Unary
impl_op_ex!(- |a: Vector4f| -> Vector4f { Vector4f { x: -a.x, y: -a.y, z: -a.z, w: -a.w } });