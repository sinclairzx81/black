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

use super::Vec3;
use super::Vec4;
use super::Mat4;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    #[inline(always)]
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
    #[inline(always)]
    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    #[inline(always)]
    pub fn one() -> Vec2 {
        Vec2::new(1.0, 1.0)
    }
    #[inline(always)]
    pub fn unit_x() -> Vec2 {
        Vec2::new(1.0, 0.0)
    }
    #[inline(always)]
    pub fn unit_y() -> Vec2 {
        Vec2::new(0.0, 1.0)
    }
    #[inline(always)]
    pub fn equals(v0: &Vec2, v1: &Vec2) -> bool {
        v0.x == v1.x && v0.y == v1.y
    }
    #[inline(always)]
    pub fn add(v0: &Vec2, v1: &Vec2) -> Vec2 {
        Vec2::new(v0.x + v1.x, v0.y + v1.y)
    }
    #[inline(always)]
    pub fn sub(v0: &Vec2, v1: &Vec2) -> Vec2 {
        Vec2::new(v0.x - v1.x, v0.y - v1.y)
    }
    #[inline(always)]
    pub fn mul(v0: &Vec2, v1: &Vec2) -> Vec2 {
        Vec2::new(v0.x * v1.x, v0.y * v1.y)
    }
    #[inline(always)]
    pub fn div(v0: &Vec2, v1: &Vec2) -> Vec2 {
        Vec2::new(v0.x / v1.x, v0.y / v1.y)
    }
    #[inline(always)]
    pub fn scale(v0: &Vec2, s: f32) -> Vec2 {
        Vec2::new(v0.x * s, v0.y * s)
    }
    #[inline(always)]
    pub fn negate(v0: &Vec2) -> Vec2 {
        Vec2::new(-v0.x, -v0.y)
    }
    #[inline(always)]
    pub fn length(v0: &Vec2) -> f32 {
        f32::sqrt((v0.x * v0.x) + (v0.y * v0.y))
    }
    #[inline(always)]
    pub fn length_sq(v0: &Vec2) -> f32 {
        (v0.x * v0.x) + (v0.y * v0.y)
    }
    #[inline(always)]
    pub fn distance(v0: &Vec2, v1: &Vec2) -> f32 {
        let d = Vec2::sub(v0, v1);
        Vec2::length(&d)
    }
    #[inline(always)]
    pub fn distance_sq(v0: &Vec2, v1: &Vec2) -> f32 {
        let d = Vec2::sub(v0, v1);
        Vec2::length_sq(&d)
    }
    #[inline(always)]
    pub fn dot(v0: &Vec2, v1: &Vec2) -> f32 {
        (v0.x * v1.x) + (v0.y * v1.y)
    }
    #[inline(always)]
    pub fn normalize(v0: &Vec2) -> Vec2 {
        let len = 1.0 / f32::sqrt((v0.x * v0.x) + (v0.y * v0.y));
        Vec2::new(v0.x * len, v0.y * len)
    }
    #[inline(always)]
    pub fn abs(v0: &Vec2) -> Vec2 {
        Vec2::new(
            f32::abs(v0.x),
            f32::abs(v0.y),
        )
    }
    #[inline(always)]
    pub fn mod_f32(v0: &Vec2, v: f32) -> Vec2 {
        Vec2::new(
            v0.x % v,
            v0.y % v,
        )
    }
    #[inline(always)]
    pub fn min(v0: &Vec2, v1: &Vec2) -> Vec2 {
        Vec2::new(
            if v0.x < v1.x { v0.x } else { v1.x },
            if v0.y < v1.y { v0.y } else { v1.y },
        )
    }
    #[inline(always)]
    pub fn max(v0: &Vec2, v1: &Vec2) -> Vec2 {
        Vec2::new(
            if v0.x > v1.x { v0.x } else { v1.x },
            if v0.y > v1.y { v0.y } else { v1.y },
        )
    }
    #[inline(always)]
    pub fn clamp(v0: &Vec2, min: &Vec2, max: &Vec2) -> Vec2 {
        let mut x = v0.x;
        let mut y = v0.y;
        x = if x > max.x { max.x } else { x };
        x = if x < min.x { min.x } else { x };
        y = if y > max.y { max.y } else { y };
        y = if y < min.y { min.y } else { y };
        Vec2::new(x, y)
    }
    #[inline(always)]
    pub fn lerp(v0: &Vec2, v1: &Vec2, amount: f32) -> Vec2 {
        Vec2::new(
            v0.x + ((v1.x - v0.x) * amount),
            v0.y + ((v1.y - v0.y) * amount),
        )
    }
    #[inline(always)]
    pub fn barycentric(
        v0: &Vec2,
        v1: &Vec2,
        v2: &Vec2,
        amount0: f32,
        amount1: f32,
    ) -> Vec2 {
        Vec2::new(
            (v0.x + (amount0 * (v1.x - v0.x))) + (amount1 * (v2.x - v0.x)),
            (v0.y + (amount0 * (v1.y - v0.y))) + (amount1 * (v2.y - v0.y)),
        )
    }
    #[inline(always)]
    pub fn smooth_step(v0: &Vec2, v1: &Vec2, amount: f32) -> Vec2 {
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
        Vec2::new(
            v0.x + ((v1.x - v0.x) * amount),
            v0.y + ((v1.y - v0.y) * amount),
        )
    }
    #[inline(always)]
    pub fn catmull_rom(
        v0: &Vec2,
        v1: &Vec2,
        v2: &Vec2,
        v3: &Vec2,
        amount: f32,
    ) -> Vec2 {
        let n0 = amount * amount;
        let n1 = amount * n0;
        Vec2::new(
            0.5 * ((((2.0 * v1.x) + ((-v0.x + v2.x) * amount))
                + (((((2.0 * v0.x) - (5.0 * v1.x)) + (4.0 * v2.x)) - v3.x) * n0))
                + ((((-v0.x + (3.0 * v1.x)) - (3.0 * v2.x)) + v3.x) * n1)),
            0.5 * ((((2.0 * v1.y) + ((-v0.y + v2.y) * amount))
                + (((((2.0 * v0.y) - (5.0 * v1.y)) + (4.0 * v2.y)) - v3.y) * n0))
                + ((((-v0.y + (3.0 * v1.y)) - (3.0 * v2.y)) + v3.y) * n1)),
        )
    }
    #[inline(always)]
    pub fn hermite(v0: &Vec2, t0: &Vec2, v1: &Vec2, t1: &Vec2, amount: f32) -> Vec2 {
        let n0 = amount * amount;
        let n1 = amount * n0;
        let n2 = ((2.0 * n1) - (3.0 * n0)) + 1.0;
        let n3 = (-2.0 * n1) + (3.0 * n0);
        let n4 = (n1 - (2.0 * n0)) + amount;
        let n5 = n1 - n0;
        Vec2::new(
            (((v0.x * n2) + (v1.x * n3)) + (t0.x * n4)) + (t1.x * n5),
            (((v0.y * n2) + (v1.y * n3)) + (t0.y * n4)) + (t1.y * n5),
        )
    }
    #[inline(always)]
    pub fn transform(v0: &Vec2, m0: &Mat4) -> Vec2 {
        Vec2::new(
            ((v0.x * m0.m11) + (v0.y * m0.m21)) + m0.m41,
            ((v0.x * m0.m12) + (v0.y * m0.m22)) + m0.m42
        )
    }
    #[inline(always)]
    pub fn transform_normal(n0: &Vec2, m0: &Mat4) -> Vec2 {
        Vec2::new(
            (n0.x * m0.m11) + (n0.y * m0.m21),
            (n0.x * m0.m12) + (n0.y * m0.m22)
        )
    }
}

// ------------------------------------------------------------
//
// Equality Operator
//
// ------------------------------------------------------------

impl PartialEq for Vec2 {
    fn eq(&self, rhs: &Self) -> bool {
        Vec2::equals(self, rhs)
    }
}
impl Eq for Vec2 {}

// ------------------------------------------------------------
//
// Operator Overloads: &T + &T
//
// ------------------------------------------------------------

impl Add<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn add(self, rhs: &Vec2) -> Vec2 {
        Vec2::add(self, rhs)
    }
}
impl Sub<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: &Vec2) -> Vec2 {
        Vec2::sub(self, rhs)
    }
}
impl Mul<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: &Vec2) -> Vec2 {
        Vec2::mul(self, rhs)
    }
}
impl Mul<f32> for &Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2::scale(self, rhs)
    }
}
impl Div<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn div(self, rhs: &Vec2) -> Vec2 {
        Vec2::div(self, rhs)
    }
}

// ------------------------------------------------------------
//
// Operator Overloads: T + T
//
// ------------------------------------------------------------

impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::add(&self, &rhs)
    }
}
impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::sub(&self, &rhs)
    }
}
impl Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::mul(&self, &rhs)
    }
}
impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2::scale(&self, rhs)
    }
}
impl Div<Vec2> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2::div(&self, &rhs)
    }
}

// ------------------------------------------------------------
//
// Indexer
//
// ------------------------------------------------------------
impl Index<usize> for Vec2 {
    type Output = f32;
    fn index<'a>(&'a self, i: usize) -> &'a f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2: Index out of range")
        }
    }
}
impl IndexMut<usize> for Vec2 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vec2: Index out of range")
        }
       
    }
}
// ------------------------------------------------------------
//
// Display
//
// ------------------------------------------------------------

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Vec2 {{ x: {}, y: {} }}",
            self.x, self.y
        )
    }
}
