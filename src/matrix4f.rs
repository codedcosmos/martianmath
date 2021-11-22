use std::{fmt, ops};
use impl_ops::*;

#[derive(Copy, Clone, Debug)]
pub struct Matrix4f {
	pub m00: f32,
	pub m10: f32,
	pub m20: f32,
	pub m30: f32,

	pub m01: f32,
	pub m11: f32,
	pub m21: f32,
	pub m31: f32,

	pub m02: f32,
	pub m12: f32,
	pub m22: f32,
	pub m32: f32,

	pub m03: f32,
	pub m13: f32,
	pub m23: f32,
	pub m33: f32,
}

// Custom
impl Matrix4f {
	pub fn new() -> Self {
		Self {
			m00: 0.0,
			m10: 0.0,
			m20: 0.0,
			m30: 0.0,

			m01: 0.0,
			m11: 0.0,
			m21: 0.0,
			m31: 0.0,

			m02: 0.0,
			m12: 0.0,
			m22: 0.0,
			m32: 0.0,

			m03: 0.0,
			m13: 0.0,
			m23: 0.0,
			m33: 0.0
		}
	}

	pub fn from(m00: f32, m10: f32, m20: f32, m30: f32,
				m01: f32, m11: f32, m21: f32, m31: f32,
				m02: f32, m12: f32, m22: f32, m32: f32,
				m03: f32, m13: f32, m23: f32, m33: f32) -> Self {
		Self {
			m00,
			m10,
			m20,
			m30,
			m01,
			m11,
			m21,
			m31,
			m02,
			m12,
			m22,
			m32,
			m03,
			m13,
			m23,
			m33
		}
	}

	pub fn identity() -> Self {
		Self {
			m00: 1.0,
			m10: 0.0,
			m20: 0.0,
			m30: 0.0,

			m01: 0.0,
			m11: 1.0,
			m21: 0.0,
			m31: 0.0,

			m02: 0.0,
			m12: 0.0,
			m22: 1.0,
			m32: 0.0,

			m03: 0.0,
			m13: 0.0,
			m23: 0.0,
			m33: 1.0
		}
	}

	pub fn transpose(&self) -> Self {
		Self {
			m00: self.m00,
			m10: self.m01,
			m20: self.m02,
			m30: self.m03,

			m01: self.m10,
			m11: self.m11,
			m21: self.m12,
			m31: self.m13,

			m02: self.m20,
			m12: self.m21,
			m22: self.m22,
			m32: self.m23,

			m03: self.m30,
			m13: self.m31,
			m23: self.m32,
			m33: self.m33
		}
	}

	pub fn display(&self) {
		println!("/{} {} {} {}\\", self.m00, self.m10, self.m20, self.m30);
		println!("|{} {} {} {}|", self.m01, self.m11, self.m21, self.m31);
		println!("|{} {} {} {}|", self.m02, self.m12, self.m22, self.m32);
		println!("\\{} {} {} {}/", self.m03, self.m13, self.m23, self.m33);
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