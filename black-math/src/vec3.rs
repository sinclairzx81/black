/*--------------------------------------------------------------------------

black

The MIT License (MIT)

Copyright (c) 2019 Haydn Paterson (sinclair) <haydn.developer@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

---------------------------------------------------------------------------*/


use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{Index, IndexMut};

use super::Vec4;
use super::Mat4;
use super::Quaternion;


#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    #[inline(always)]
    pub fn all(x: f32) -> Vec3 {
        Vec3 { x, y: x, z: x }
    }
    #[inline(always)]
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    #[inline(always)]
    pub fn one() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }
    #[inline(always)]
    pub fn unit_x() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
    #[inline(always)]
    pub fn unit_y() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }
    #[inline(always)]
    pub fn unit_z() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }
    #[inline(always)]
    pub fn equals(v0: &Vec3, v1: &Vec3) -> bool {
        v0.x == v1.x && v0.y == v1.y && v0.z == v1.z
    }
    #[inline(always)]
    pub fn add(v0: &Vec3, v1: &Vec3) -> Vec3 {
        Vec3::new(v0.x + v1.x, v0.y + v1.y, v0.z + v1.z)
    }
    #[inline(always)]
    pub fn sub(v0: &Vec3, v1: &Vec3) -> Vec3 {
        Vec3::new(v0.x - v1.x, v0.y - v1.y, v0.z - v1.z)
    }
    #[inline(always)]
    pub fn mul(v0: &Vec3, v1: &Vec3) -> Vec3 {
        Vec3::new(v0.x * v1.x, v0.y * v1.y, v0.z * v1.z)
    }
    #[inline(always)]
    pub fn div(v0: &Vec3, v1: &Vec3) -> Vec3 {
        Vec3::new(v0.x / v1.x, v0.y / v1.y, v0.z / v1.z)
    }
    #[inline(always)]
    pub fn scale(v0: &Vec3, s: f32) -> Vec3 {
        Vec3::new(v0.x * s, v0.y * s, v0.z * s)
    }
    #[inline(always)]
    pub fn negate(v0: &Vec3) -> Vec3 {
        Vec3::new(-v0.x, -v0.y, -v0.z)
    }
    #[inline(always)]
    pub fn length(v0: &Vec3) -> f32 {
        f32::sqrt((v0.x * v0.x) + (v0.y * v0.y) + (v0.z * v0.z))
    }
    #[inline(always)]
    pub fn length_sq(v0: &Vec3) -> f32 {
        (v0.x * v0.x) + (v0.y * v0.y) + (v0.z * v0.z)
    }
    #[inline(always)]
    pub fn distance(v0: &Vec3, v1: &Vec3) -> f32 {
        let d = Vec3::sub(v0, v1);
        Vec3::length(&d)
    }
    #[inline(always)]
    pub fn distance_sq(v0: &Vec3, v1: &Vec3) -> f32 {
        let d = Vec3::sub(v0, v1);
        Vec3::length_sq(&d)
    }
    #[inline(always)]
    pub fn dot(v0: &Vec3, v1: &Vec3) -> f32 {
        (v0.x * v1.x) + (v0.y * v1.y) + (v0.z * v1.z)
    }
    #[inline(always)]
    pub fn normalize(v0: &Vec3) -> Vec3 {
        let len = 1.0 / f32::sqrt((v0.x * v0.x) + (v0.y * v0.y) + (v0.z * v0.z));
        Vec3::new(v0.x * len, v0.y * len, v0.z * len)
    }
    #[inline(always)]
    pub fn cross(v0: &Vec3, v1: &Vec3) -> Vec3 {
        Vec3::new(
            (v0.y * v1.z) - (v0.z * v1.y),
            (v0.z * v1.x) - (v0.x * v1.z),
            (v0.x * v1.y) - (v0.y * v1.x)
        )
    }
    #[inline(always)]
    pub fn reflect(v0: &Vec3, n0: &Vec3) -> Vec3 {
        let dot = Vec3::dot(v0, n0);
        Vec3::new(
            v0.x - ((2.0 * dot) * n0.x),
            v0.y - ((2.0 * dot) * n0.y),
            v0.z - ((2.0 * dot) * n0.z),
        )
    }
    #[inline(always)]
    pub fn abs(v0: &Vec3) -> Vec3 {
        Vec3::new(
            f32::abs(v0.x),
            f32::abs(v0.y),
            f32::abs(v0.z),
        )
    }
    #[inline(always)]
    pub fn mod_f32(v0: &Vec3, v: f32) -> Vec3 {
        Vec3::new(
            v0.x % v,
            v0.y % v,
            v0.z % v,
        )
    }
    #[inline(always)]
    pub fn min(v0: &Vec3, v1: &Vec3) -> Vec3 {
        Vec3::new(
            if v0.x < v1.x { v0.x } else { v1.x },
            if v0.y < v1.y { v0.y } else { v1.y },
            if v0.z < v1.z { v0.z } else { v1.z },
        )
    }
    #[inline(always)]
    pub fn max(v0: &Vec3, v1: &Vec3) -> Vec3 {
        Vec3::new(
            if v0.x > v1.x { v0.x } else { v1.x },
            if v0.y > v1.y { v0.y } else { v1.y },
            if v0.z > v1.z { v0.z } else { v1.z },
        )
    }
    #[inline(always)]
    pub fn clamp(v0: &Vec3, min: &Vec3, max: &Vec3) -> Vec3 {
        let mut x = v0.x;
        let mut y = v0.y;
        let mut z = v0.z;
        x = if x > max.x { max.x } else { x };
        x = if x < min.x { min.x } else { x };
        y = if y > max.y { max.y } else { y };
        y = if y < min.y { min.y } else { y };
        z = if z > max.z { max.z } else { z };
        z = if z < min.z { min.z } else { z };
        Vec3::new(x, y, z)
    }
    #[inline(always)]
    pub fn lerp(v0: &Vec3, v1: &Vec3, amount: f32) -> Vec3 {
        Vec3::new(
            v0.x + ((v1.x - v0.x) * amount),
            v0.y + ((v1.y - v0.y) * amount),
            v0.z + ((v1.z - v0.z) * amount),
        )
    }
    #[inline(always)]
    pub fn barycentric(
        v0: &Vec3,
        v1: &Vec3,
        v2: &Vec3,
        amount0: f32,
        amount1: f32,
    ) -> Vec3 {
        Vec3::new(
            (v0.x + (amount0 * (v1.x - v0.x))) + (amount1 * (v2.x - v0.x)),
            (v0.y + (amount0 * (v1.y - v0.y))) + (amount1 * (v2.y - v0.y)),
            (v0.z + (amount0 * (v1.z - v0.z))) + (amount1 * (v2.z - v0.z)),
        )
    }
    #[inline(always)]
    pub fn smooth_step(v0: &Vec3, v1: &Vec3, amount: f32) -> Vec3 {
        let mut amount = if amount > 1.0 {
            1.0
        } else {
            if amount < 0.0 {
                0.0
            } else {
                amount
            }
        };
        amount = (amount * amount) * (3.0 - (2.0 * amount));
        Vec3::new(
            v0.x + ((v1.x - v0.x) * amount),
            v0.y + ((v1.y - v0.y) * amount),
            v0.z + ((v1.z - v0.z) * amount),
        )
    }
    #[inline(always)]
    pub fn catmull_rom(
        v0: &Vec3,
        v1: &Vec3,
        v2: &Vec3,
        v3: &Vec3,
        amount: f32,
    ) -> Vec3 {
        let n0 = amount * amount;
        let n1 = amount * n0;
        Vec3::new(
            0.5 * ((((2.0 * v1.x) + ((-v0.x + v2.x) * amount))
                + (((((2.0 * v0.x) - (5.0 * v1.x)) + (4.0 * v2.x)) - v3.x) * n0))
                + ((((-v0.x + (3.0 * v1.x)) - (3.0 * v2.x)) + v3.x) * n1)),
            0.5 * ((((2.0 * v1.y) + ((-v0.y + v2.y) * amount))
                + (((((2.0 * v0.y) - (5.0 * v1.y)) + (4.0 * v2.y)) - v3.y) * n0))
                + ((((-v0.y + (3.0 * v1.y)) - (3.0 * v2.y)) + v3.y) * n1)),
            0.5 * ((((2.0 * v1.z) + ((-v0.z + v2.z) * amount))
                + (((((2.0 * v0.z) - (5.0 * v1.z)) + (4.0 * v2.z)) - v3.z) * n0))
                + ((((-v0.z + (3.0 * v1.z)) - (3.0 * v2.z)) + v3.z) * n1)),
        )
    }
    #[inline(always)]
    pub fn hermite(v0: &Vec3, t0: &Vec3, v1: &Vec3, t1: &Vec3, amount: f32) -> Vec3 {
        let n0 = amount * amount;
        let n1 = amount * n0;
        let n2 = ((2.0 * n1) - (3.0 * n0)) + 1.0;
        let n3 = (-2.0 * n1) + (3.0 * n0);
        let n4 = (n1 - (2.0 * n0)) + amount;
        let n5 = n1 - n0;
        Vec3::new(
            (((v0.x * n2) + (v1.x * n3)) + (t0.x * n4)) + (t1.x * n5),
            (((v0.y * n2) + (v1.y * n3)) + (t0.y * n4)) + (t1.y * n5),
            (((v0.z * n2) + (v1.z * n3)) + (t0.z * n4)) + (t1.z * n5),
        )
    }
    #[inline(always)]
    pub fn transform(v0: &Vec3, m0: &Mat4) -> Vec3 {
        Vec3::new(
            ((v0.x * m0.m11) + (v0.y * m0.m21)) + (v0.z * m0.m31),
            ((v0.x * m0.m12) + (v0.y * m0.m22)) + (v0.z * m0.m32),
            ((v0.x * m0.m13) + (v0.y * m0.m23)) + (v0.z * m0.m33),
        )
    }
    #[inline(always)]
    pub fn transform_normal(n0: &Vec3, m0: &Mat4) -> Vec3 {
        Vec3::new(
            ((n0.x * m0.m11) + (n0.y * m0.m21)) + (n0.z * m0.m31),
            ((n0.x * m0.m12) + (n0.y * m0.m22)) + (n0.z * m0.m32),
            ((n0.x * m0.m13) + (n0.y * m0.m23)) + (n0.z * m0.m33),
        )
    }
    #[inline(always)]
    pub fn transform_quaternion(v0: &Vec3, q0: &Quaternion) -> Vec3 {
        let n0 = q0.x + q0.x;
        let n1 = q0.y + q0.y;
        let n2 = q0.z + q0.z;
        let n3 = q0.w * n0;
        let n4 = q0.w * n1;
        let n5 = q0.w * n2;
        let n6 = q0.x * n0;
        let n7 = q0.x * n1;
        let n8 = q0.x * n2;
        let n9 = q0.y * n1;
        let n10 = q0.y * n2;
        let n11 = q0.z * n2;
        Vec3::new(
            (v0.x * ((1.0 - n9) - n11)) + (v0.y * (n7 - n5)),
            (v0.x * (n7 + n5)) + (v0.y * ((1.0 - n6) - n11)),
            (v0.x * (n8 - n4)) + (v0.y * (n10 + n3)),
        )
    }
}

// ------------------------------------------------------------
//
// Instance functions.
//
// ------------------------------------------------------------

impl Vec3 {
    pub fn xyzw(&self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, 1.0)
    }
}

// ------------------------------------------------------------
//
// Equality Operator
//
// ------------------------------------------------------------

impl PartialEq for Vec3 {
    fn eq(&self, rhs: &Self) -> bool {
        Vec3::equals(self, rhs)
    }
}
impl Eq for Vec3 {}

// ------------------------------------------------------------
//
// Operator Overloads: &T + &T
//
// ------------------------------------------------------------

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3::add(self, rhs)
    }
}
impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3::sub(self, rhs)
    }
}
impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3::mul(self, rhs)
    }
}
impl Mul<f32> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::scale(self, rhs)
    }
}
impl Div<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: &Vec3) -> Vec3 {
        Vec3::div(self, rhs)
    }
}

// ------------------------------------------------------------
//
// Operator Overloads: T + T
//
// ------------------------------------------------------------

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::add(&self, &rhs)
    }
}
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::sub(&self, &rhs)
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::mul(&self, &rhs)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::scale(&self, rhs)
    }
}
impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::div(&self, &rhs)
    }
}

// ------------------------------------------------------------
//
// Indexer
//
// ------------------------------------------------------------
impl Index<usize> for Vec3 {
    type Output = f32;
    fn index<'a>(&'a self, i: usize) -> &'a f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3: Index out of range")
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3: Index out of range")
        }
       
    }
}

// ------------------------------------------------------------
//
// Display
//
// ------------------------------------------------------------
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Vec3 {{ x: {}, y: {}, z: {} }}",
            self.x, self.y, self.z,
        )
    }
}
