use std::{fmt, ops};
use impl_ops::*;
use crate::vector3f::Vector3f;

#[derive(Copy, Clone, Debug)]
pub struct Vector2f {
	pub x: f32,
	pub y: f32,
}

// Custom
impl Vector2f {
	pub fn new() -> Self {
		Self {
			x: 0.0,
			y: 0.0
		}
	}

	pub fn from(x: f32, y: f32) -> Self {
		Self {
			x,
			y
		}
	}

	pub fn max(&self) -> f32 {
		self.x.max(self.y)
	}

	pub fn min(&self) -> f32 {
		self.x.min(self.y)
	}

	pub fn absolute(&self) -> Vector2f {
		Self {
			x: self.x.abs(),
			y: self.y.abs()
		}
	}

	pub fn to_homogeneous(&self) -> Vector3f {
		Vector3f::from(self.x, self.y, 1.0)
	}

	pub fn dot(&self, b: Self) -> f32 {
		self.x*b.x + self.y*b.y
	}

	pub fn length(&self) -> f32 {
		(self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
	}

	pub fn normalise(&self) -> Self {
		let len = self.length();

		Self {
			x: self.x / len,
			y: self.y / len
		}
	}
}

// Display
impl fmt::Display for Vector2f {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Use `self.number` to refer to each positional data point.
		write!(f, "<{}, {}>", self.x, self.y)
	}
}

// Add subtract
impl_op_ex!(+ |a: Vector2f, b: Vector2f| -> Vector2f { Vector2f { x: a.x + b.x, y: a.y + b.y } });
impl_op_ex!(- |a: Vector2f, b: Vector2f| -> Vector2f { Vector2f { x: a.x - b.x, y: a.y - b.y } });

// Scalar operations
impl_op_ex!(+ |a: Vector2f, b: f32| -> Vector2f { Vector2f { x: a.x + b, y: a.y + b } });
impl_op_ex!(+ |a: f32, b: Vector2f| -> Vector2f { Vector2f { x: a + b.x, y: a + b.y } });
impl_op_ex!(- |a: Vector2f, b: f32| -> Vector2f { Vector2f { x: a.x - b, y: a.y - b } });
impl_op_ex!(- |a: f32, b: Vector2f| -> Vector2f { Vector2f { x: a - b.x, y: a - b.y } });
impl_op_ex!(* |a: Vector2f, b: f32| -> Vector2f { Vector2f { x: a.x * b, y: a.y * b } });
impl_op_ex!(* |a: f32, b: Vector2f| -> Vector2f { Vector2f { x: a * b.x, y: a * b.y } });
impl_op_ex!(/ |a: Vector2f, b: f32| -> Vector2f { Vector2f { x: a.x / b, y: a.y / b } });
impl_op_ex!(/ |a: f32, b: Vector2f| -> Vector2f { Vector2f { x: a / b.x, y: a / b.y } });

// Unary
impl_op_ex!(- |a: Vector2f| -> Vector2f { Vector2f { x: -a.x, y: -a.y} });