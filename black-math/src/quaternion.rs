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

use super::Mat4;
use super::Vec3;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Quaternion {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion { x, y, z, w }
    }
    #[inline(always)]
    pub fn equals(q0: &Quaternion, q1: &Quaternion) -> bool {
        q0.x == q1.x && q0.y == q1.y && q0.z == q1.z && q0.w == q1.w
    }
    #[inline(always)]
    pub fn length(q0: &Quaternion) -> f32 {
        f32::sqrt((q0.x * q0.x) + (q0.y * q0.y) + (q0.z * q0.z) + (q0.w * q0.w))
    }
    #[inline(always)]
    pub fn length_sq(q0: &Quaternion) -> f32 {
        (q0.x * q0.x) + (q0.y * q0.y) + (q0.z * q0.z) + (q0.w * q0.w)
    }
    #[inline(always)]
    pub fn normalize(q0: &Quaternion) -> Quaternion {
        let len = 1.0 / f32::sqrt((q0.x * q0.x) + (q0.y * q0.y) + (q0.z * q0.z) + (q0.w * q0.w));
        Quaternion::new(q0.x * len, q0.y * len, q0.z * len, q0.w * len)
    }
    #[inline(always)]
    pub fn dot(q0: &Quaternion, q1: &Quaternion) -> f32 {
        (q0.x * q1.x) + (q0.y * q1.y) + (q0.z * q1.z) + (q0.w * q1.w)
    }
    #[inline(always)]
    pub fn conjugate(q0: &Quaternion) -> Quaternion {
        Quaternion::new(-q0.x, -q0.y, -q0.z, q0.w)
    }
    #[inline(always)]
    pub fn inverse(q0: &Quaternion) -> Quaternion {
        let n0 = (((q0.x * q0.x) + (q0.y * q0.y)) + (q0.z * q0.z)) + (q0.w * q0.w);
        let n1 = 1.0 / n0;
        Quaternion::new(-q0.x * n1, -q0.y * n1, -q0.z * n1, -q0.w * n1)
    }
    #[inline(always)]
    pub fn slerp(q0: &Quaternion, q1: &Quaternion, amount: f32) -> Quaternion {
        let n0;
        let n1;
        let n2 = amount;
        let mut n3 = (((q0.x * q1.x) + (q0.y * q1.y)) + (q0.z * q1.z)) + (q0.w * q1.w);
        let mut flag = false;
        if n3 < 0.0 {
            flag = true;
            n3 = -n3;
        }
        if n3 > 0.999999 {
            n1 = 1.0 - n2;
            n0 = if flag { -n2 } else { n2 }
        } else {
            let n4 = f32::acos(n3);
            let n5 = 1.0 / f32::sin(n4);
            n1 = f32::sin((1.0 - n2) * n4) * n5;
            n0 = if flag {
                -f32::sin(n2 * n4) * n5
            } else {
                f32::sin(n2 * n4) * n5
            };
        }
        Quaternion::new(
            (n1 * q0.x) + (n0 * q1.x),
            (n1 * q0.y) + (n0 * q1.y),
            (n1 * q0.z) + (n0 * q1.z),
            (n1 * q0.w) + (n0 * q1.w),
        )
    }
    #[inline(always)]
    pub fn lerp(q0: &Quaternion, q1: &Quaternion, amount: f32) -> Quaternion {
        let mut q2 = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        let n0 = amount;
        let n1 = 1.0 - n0;
        let n2 = (((q0.x * q1.x) + (q0.y * q1.y)) + (q0.z * q1.z)) + (q0.w * q1.w);
        if n2 >= 0.0 {
            q2.x = (n1 * q0.x) + (n0 * q1.x);
            q2.y = (n1 * q0.y) + (n0 * q1.y);
            q2.z = (n1 * q0.z) + (n0 * q1.z);
            q2.w = (n1 * q0.w) + (n0 * q1.w);
        } else {
            q2.x = (n1 * q0.x) - (n0 * q1.x);
            q2.y = (n1 * q0.y) - (n0 * q1.y);
            q2.z = (n1 * q0.z) - (n0 * q1.z);
            q2.w = (n1 * q0.w) - (n0 * q1.w);
        }
        let n3 = (((q2.x * q2.x) + (q2.y * q2.y)) + (q2.z * q2.z)) + (q2.w * q2.w);
        let n4 = 1.0 / f32::sqrt(n3);
        q2.x *= n4;
        q2.y *= n4;
        q2.z *= n4;
        q2.w *= n4;
        q2
    }
    #[inline(always)]
    pub fn from_axis_angle(v0: &Vec3, angle: f32) -> Quaternion {
        let n0 = angle * 0.5;
        let n1 = f32::sin(n0);
        let n2 = f32::cos(n0);
        Quaternion::new(v0.x * n1, v0.y * n1, v0.z * n1, n2)
    }
    #[inline(always)]
    pub fn from_matrix(m0: &Mat4) -> Quaternion {
        let n0 = (m0.m11 + m0.m22) + m0.m33;
        if n0 > 0.0 {
            let n1 = f32::sqrt(n0 + 1.0);
            let n2 = 0.5 / n1;
            Quaternion::new(
                (m0.m23 - m0.m32) * n2,
                (m0.m31 - m0.m13) * n2,
                (m0.m12 - m0.m21) * n2,
                n1 * 0.5,
            )
        } else if (m0.m11 >= m0.m22) && (m0.m11 >= m0.m33) {
            let n1 = f32::sqrt(((1.0 + m0.m11) - m0.m22) - m0.m33);
            let n2 = 0.5 / n1;
            Quaternion::new(
                0.5 * n1,
                (m0.m12 + m0.m21) * n2,
                (m0.m13 + m0.m31) * n2,
                (m0.m23 - m0.m32) * n2,
            )
        } else if m0.m22 > m0.m33 {
            let n1 = f32::sqrt(((1.0 + m0.m22) - m0.m11) - m0.m33);
            let n2 = 0.5 / n1;
            Quaternion::new(
                (m0.m21 + m0.m12) * n2,
                0.5 * n1,
                (m0.m32 + m0.m23) * n2,
                (m0.m31 - m0.m13) * n2,
            )
        } else {
            let n1 = f32::sqrt(((1.0 + m0.m33) - m0.m11) - m0.m22);
            let n2 = 0.5 / n1;
            Quaternion::new(
                (m0.m31 + m0.m13) * n2,
                (m0.m32 + m0.m23) * n2,
                0.5 * n1,
                (m0.m12 - m0.m21) * n2,
            )
        }
    }
    #[inline(always)]
    pub fn concat(q0: &Quaternion, q1: &Quaternion) -> Quaternion {
        let n0 = q1.x;
        let n1 = q1.y;
        let n2 = q1.z;
        let n3 = q1.w;
        let n4 = q0.x;
        let n5 = q0.y;
        let n6 = q0.z;
        let n7 = q0.w;
        let n8 = (n1 * n6) - (n2 * n5);
        let n9 = (n2 * n4) - (n0 * n6);
        let n10 = (n0 * n5) - (n1 * n4);
        let n11 = ((n0 * n4) + (n1 * n5)) + (n2 * n6);
        Quaternion::new(
            ((n0 * n7) + (n4 * n3)) + n8,
            ((n1 * n7) + (n5 * n3)) + n9,
            ((n2 * n7) + (n6 * n3)) + n10,
            (n3 * n7) - n11,
        )
    }
    #[inline(always)]
    pub fn add(q0: &Quaternion, q1: &Quaternion) -> Quaternion {
        Quaternion::new(q0.x + q1.x, q0.y + q1.y, q0.z + q1.z, q0.w + q1.w)
    }
    #[inline(always)]
    pub fn sub(q0: &Quaternion, q1: &Quaternion) -> Quaternion {
        Quaternion::new(q0.x - q1.x, q0.y - q1.y, q0.z - q1.z, q0.w - q1.w)
    }
    #[inline(always)]
    pub fn mul(q0: &Quaternion, q1: &Quaternion) -> Quaternion {
        let n0 = q0.x;
        let n1 = q0.y;
        let n2 = q0.z;
        let n3 = q0.w;
        let n4 = q1.x;
        let n5 = q1.y;
        let n6 = q1.z;
        let n7 = q1.w;
        let n8 = (n1 * n6) - (n2 * n5);
        let n9 = (n2 * n4) - (n0 * n6);
        let n10 = (n0 * n5) - (n1 * n4);
        let n11 = ((n0 * n4) + (n1 * n5)) + (n2 * n6);
        Quaternion::new(
            ((n0 * n7) + (n4 * n3)) + n8,
            ((n1 * n7) + (n5 * n3)) + n9,
            ((n2 * n7) + (n6 * n3)) + n10,
            (n3 * n7) - n11,
        )
    }
    #[inline(always)]
    pub fn div(q0: &Quaternion, q1: &Quaternion) -> Quaternion {
        let n0 = q0.x;
        let n1 = q0.y;
        let n2 = q0.z;
        let n3 = q0.w;
        let n4 = (((q1.x * q1.x) + (q1.y * q1.y)) + (q1.z * q1.z)) + (q1.w * q1.w);
        let n5 = 1.0 / n4;
        let n6 = -q1.x * n5;
        let n7 = -q1.y * n5;
        let n8 = -q1.z * n5;
        let n9 = q1.w * n5;
        let n10 = (n1 * n8) - (n2 * n7);
        let n11 = (n2 * n6) - (n0 * n8);
        let n12 = (n0 * n7) - (n1 * n6);
        let n13 = ((n0 * n6) + (n1 * n7)) + (n2 * n8);
        Quaternion::new(
            ((n0 * n9) + (n6 * n3)) + n10,
            ((n1 * n9) + (n7 * n3)) + n11,
            ((n2 * n9) + (n8 * n3)) + n12,
            (n3 * n9) - n13,
        )
    }
    pub fn negate(q0: &Quaternion) -> Quaternion {
        Quaternion::new(-q0.x, -q0.y, -q0.z, -q0.w)
    }
}
// ------------------------------------------------------------
//
// Equality Operator
//
// ------------------------------------------------------------

impl PartialEq for Quaternion {
    fn eq(&self, rhs: &Self) -> bool {
        Quaternion::equals(self, rhs)
    }
}
impl Eq for Quaternion {}


// ------------------------------------------------------------
//
// Operator Overloads: &T + &T
//
// ------------------------------------------------------------

impl Add<&Quaternion> for &Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: &Quaternion) -> Quaternion {
        Quaternion::add(self, rhs)
    }
}
impl Sub<&Quaternion> for &Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: &Quaternion) -> Quaternion {
        Quaternion::sub(self, rhs)
    }
}
impl Mul<&Quaternion> for &Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: &Quaternion) -> Quaternion {
        Quaternion::mul(self, rhs)
    }
}
impl Div<&Quaternion> for &Quaternion {
    type Output = Quaternion;
    fn div(self, rhs: &Quaternion) -> Quaternion {
        Quaternion::div(self, rhs)
    }
}

// ------------------------------------------------------------
//
// Operator Overloads: T + T
//
// ------------------------------------------------------------

impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Quaternion {
        Quaternion::add(&self, &rhs)
    }
}
impl Sub<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: Quaternion) -> Quaternion {
        Quaternion::sub(&self, &rhs)
    }
}
impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion::mul(&self, &rhs)
    }
}
impl Div<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn div(self, rhs: Quaternion) -> Quaternion {
        Quaternion::div(&self, &rhs)
    }
}
// ------------------------------------------------------------
//
// Display
//
// ------------------------------------------------------------
impl Display for Quaternion {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Quaternion {{ x: {}, y: {}, z: {}, w: {} }}",
            self.x, self.y, self.z, self.w
        )
    }
}
