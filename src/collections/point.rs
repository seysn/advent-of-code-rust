use std::{
	fmt::{Debug, Display, Write},
	ops::{Add, AddAssign, Mul, MulAssign, Neg},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3D(pub i32, pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector(pub i32, pub i32);

impl Point {
	pub fn vector(&self, other: &Self) -> Vector {
		Vector(other.0 - self.0, other.1 - self.1)
	}
}

impl Vector {
	pub const NORTH: Self = Self(0, -1);
	pub const EAST: Self = Self(1, 0);
	pub const SOUTH: Self = Self(0, 1);
	pub const WEST: Self = Self(-1, 0);

	pub const NORTH_EAST: Self = Self(1, -1);
	pub const SOUTH_EAST: Self = Self(1, 1);
	pub const SOUTH_WEST: Self = Self(-1, 1);
	pub const NORTH_WEST: Self = Self(-1, -1);

	/// North, East, South, East
	pub const CARDINAL: [Self; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];
	/// North-East, South-East, South-West, North-West
	pub const ORDINAL: [Self; 4] = [Self::NORTH_EAST, Self::SOUTH_EAST, Self::SOUTH_WEST, Self::NORTH_WEST];
	/// North, North-East, East, South-East, South, South-West, West, North-West
	pub const DIRECTIONS: [Self; 8] = [
		Self::NORTH,
		Self::NORTH_EAST,
		Self::EAST,
		Self::SOUTH_EAST,
		Self::SOUTH,
		Self::SOUTH_WEST,
		Self::WEST,
		Self::NORTH_WEST,
	];
}

impl From<char> for Vector {
	fn from(value: char) -> Self {
		match value {
			'v' | 'D' | 'S' => Self::SOUTH,
			'^' | 'U' | 'N' => Self::NORTH,
			'<' | 'L' | 'W' => Self::WEST,
			'>' | 'R' | 'E' => Self::EAST,
			_ => unimplemented!("{value}"),
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

	#[inline]
	pub fn manhattan_distance(&self) -> u32 {
		(self.0.abs() + self.1.abs()) as u32
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

impl Mul<i32> for &Vector {
	type Output = Vector;

	fn mul(self, rhs: i32) -> Self::Output {
		Vector(self.0 * rhs, self.1 * rhs)
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

impl Neg for Vector {
	type Output = Vector;

	fn neg(self) -> Self::Output {
		Vector(-self.0, -self.1)
	}
}

impl Display for Vector {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Vector::NORTH => f.write_char('^')?,
			Vector::EAST => f.write_char('>')?,
			Vector::SOUTH => f.write_char('v')?,
			Vector::WEST => f.write_char('<')?,
			_ => Debug::fmt(self, f)?,
		}

		Ok(())
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
