use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3D(pub i32, pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector(pub i32, pub i32);

impl Vector {
	pub const NORTH: Self = Self(0, -1);
	pub const EAST: Self = Self(1, 0);
	pub const SOUTH: Self = Self(0, 1);
	pub const WEST: Self = Self(-1, 0);

	pub const NORTH_EAST: Self = Self(1, -1);
	pub const SOUTH_EAST: Self = Self(1, 1);
	pub const SOUTH_WEST: Self = Self(-1, 1);
	pub const NORTH_WEST: Self = Self(-1, -1);
}

impl From<char> for Vector {
	fn from(value: char) -> Self {
		match value {
			'v' | 'D' | 'S' => Self::SOUTH,
			'^' | 'U' | 'N' => Self::NORTH,
			'<' | 'L' | 'W' => Self::WEST,
			'>' | 'R' | 'E' => Self::EAST,
			_ => unimplemented!(),
		}
	}
}

impl Vector {
	#[inline]
	pub fn reverse(&self) -> Self {
		Self(-self.0, -self.1)
	}

	#[inline]
	pub fn clockwise(&self) -> Self {
		Self(-self.1, self.0)
	}

	#[inline]
	pub fn counter_clockwise(&self) -> Self {
		Self(self.1, -self.0)
	}
}

macro_rules! impl_add {
	($lhs:ty, $rhs:ty) => {
		impl Add<$rhs> for $lhs {
			type Output = Point;

			fn add(self, rhs: $rhs) -> Self::Output {
				Point(self.0 + rhs.0, self.1 + rhs.1)
			}
		}
	};
}

impl_add!(Point, Point);
impl_add!(Point, &Point);
impl_add!(Point, Vector);
impl_add!(Point, &Vector);
impl_add!(&Point, Point);
impl_add!(&Point, &Point);
impl_add!(&Point, Vector);
impl_add!(&Point, &Vector);

impl AddAssign<Vector> for Point {
	fn add_assign(&mut self, v: Vector) {
		self.0 += v.0;
		self.1 += v.1;
	}
}

impl AddAssign<&Vector> for Point {
	fn add_assign(&mut self, v: &Vector) {
		self.0 += v.0;
		self.1 += v.1;
	}
}

impl Mul<i32> for Vector {
	type Output = Vector;

	fn mul(self, rhs: i32) -> Self::Output {
		Vector(self.0 * rhs, self.1 * rhs)
	}
}

impl MulAssign<i32> for Vector {
	fn mul_assign(&mut self, rhs: i32) {
		self.0 *= rhs;
		self.1 *= rhs;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn clockwise() {
		assert_eq!(Vector::NORTH.clockwise(), Vector::EAST);
		assert_eq!(Vector::EAST.clockwise(), Vector::SOUTH);
		assert_eq!(Vector::SOUTH.clockwise(), Vector::WEST);
		assert_eq!(Vector::WEST.clockwise(), Vector::NORTH);
		assert_eq!(Vector::NORTH.counter_clockwise(), Vector::WEST);
		assert_eq!(Vector::WEST.counter_clockwise(), Vector::SOUTH);
		assert_eq!(Vector::SOUTH.counter_clockwise(), Vector::EAST);
		assert_eq!(Vector::EAST.counter_clockwise(), Vector::NORTH);
	}
}
