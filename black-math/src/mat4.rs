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

use super::Plane;
use super::Quaternion;
use super::Vec3;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub m11: f32, pub m12: f32, pub m13: f32, pub m14: f32,
    pub m21: f32, pub m22: f32, pub m23: f32, pub m24: f32,
    pub m31: f32, pub m32: f32, pub m33: f32, pub m34: f32, 
    pub m41: f32, pub m42: f32, pub m43: f32, pub m44: f32,
}
impl Mat4 {
    #[inline(always)]
    pub fn new(m11: f32, m12: f32, m13: f32, m14: f32,
               m21: f32, m22: f32, m23: f32, m24: f32,
               m31: f32, m32: f32, m33: f32, m34: f32,
               m41: f32, m42: f32, m43: f32, m44: f32,
    ) -> Mat4 {
        Mat4 {
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44,
        }
    }
    #[inline(always)]
    pub fn equals(m0: &Mat4, m1: &Mat4) -> bool {
        m0.m11 == m1.m11 &&
        m0.m12 == m1.m12 &&
        m0.m13 == m1.m13 &&
        m0.m14 == m1.m14 &&
        m0.m21 == m1.m21 &&
        m0.m22 == m1.m22 &&
        m0.m23 == m1.m23 &&
        m0.m24 == m1.m24 &&
        m0.m31 == m1.m31 &&
        m0.m32 == m1.m32 &&
        m0.m33 == m1.m33 &&
        m0.m34 == m1.m34 &&
        m0.m41 == m1.m41 &&
        m0.m42 == m1.m42 &&
        m0.m43 == m1.m43 &&
        m0.m44 == m1.m44
    }
    #[inline(always)]
    pub fn zero() -> Mat4 {
        Mat4::new(
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        )
    }
    #[inline(always)]
    pub fn one() -> Mat4 {
        Mat4::new(
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        )
    }
    #[inline(always)]
    pub fn identity() -> Mat4 {
        Mat4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
    #[inline(always)]
    pub fn translation(v0: &Vec3) -> Mat4 {
        Mat4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, v0.x, v0.y, v0.z, 1.0,
        )
    }
    #[inline(always)]
    pub fn scale(v0: &Vec3) -> Mat4 {
        Mat4::new(
            v0.x, 0.0, 0.0, 0.0, 0.0, v0.y, 0.0, 0.0, 0.0, 0.0, v0.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
    #[inline(always)]
    pub fn rotation_x(radians: f32) -> Mat4 {
        let cos = f32::cos(radians);
        let sin = f32::sin(radians);
        Mat4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, cos, sin, 0.0, 0.0, -sin, cos, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
    #[inline(always)]
    pub fn rotation_y(radians: f32) -> Mat4 {
        let cos = f32::cos(radians);
        let sin = f32::sin(radians);
        Mat4::new(
            cos, 0.0, -sin, 0.0, 0.0, 1.0, 0.0, 0.0, sin, 0.0, cos, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
    #[inline(always)]
    pub fn rotation_z(radians: f32) -> Mat4 {
        let cos = f32::cos(radians);
        let sin = f32::sin(radians);
        Mat4::new(
            cos, sin, 0.0, 0.0, -sin, cos, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
    #[inline(always)]
    pub fn from_axis_angle(axis: &Vec3, radians: f32) -> Mat4 {
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;
        let n0 = f32::sin(radians);
        let n1 = f32::cos(radians);
        let n2 = x * x;
        let n3 = y * y;
        let n4 = z * z;
        let n5 = x * y;
        let n6 = x * z;
        let n7 = y * z;
        Mat4::new(
            n2 + (n1 * (1.0 - n2)),
            (n5 - (n1 * n5)) + (n0 * z),
            (n6 - (n1 * n6)) - (n0 * y),
            0.0,
            (n5 - (n1 * n5)) - (n0 * z),
            n3 + (n1 * (1.0 - n3)),
            (n7 - (n1 * n7)) + (n0 * x),
            0.0,
            (n6 - (n1 * n6)) + (n0 * y),
            (n7 - (n1 * n7)) - (n0 * x),
            n4 + (n1 * (1.0 - n4)),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }
    #[inline(always)]
    pub fn perspective_fov(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        let n0 = 1.0 / f32::tan(fov * 0.5);
        let n1 = n0 / aspect;
        let mut m0 = Mat4::zero();
        m0.m11 = n1;
        m0.m12 = 0.0;
        m0.m13 = 0.0;
        m0.m14 = 0.0;
        m0.m22 = n0;
        m0.m21 = 0.0;
        m0.m23 = 0.0;
        m0.m24 = 0.0;
        m0.m31 = 0.0;
        m0.m32 = 0.0;
        m0.m33 = far / (near - far);
        m0.m34 = -1.0;
        m0.m41 = 0.0;
        m0.m42 = 0.0;
        m0.m44 = 0.0;
        m0.m43 = (near * far) / (near - far);
        m0
    }
    #[inline(always)]
    pub fn perspective(width: f32, height: f32, near: f32, far: f32) -> Mat4 {
        let mut m0 = Mat4::zero();
        m0.m11 = (2.0 * near) / width;
        m0.m12 = 0.0;
        m0.m13 = 0.0;
        m0.m14 = 0.0;
        m0.m22 = (2.0 * near) / height;
        m0.m21 = 0.0;
        m0.m23 = 0.0;
        m0.m24 = 0.0;
        m0.m33 = far / (near - far);
        m0.m31 = 0.0;
        m0.m32 = 0.0;
        m0.m34 = -1.0;
        m0.m41 = 0.0;
        m0.m42 = 0.0;
        m0.m44 = 0.0;
        m0.m43 = (near * far) / (near - far);
        m0
    }
    #[inline(always)]
    pub fn perspective_offset(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Mat4 {
        let mut m0 = Mat4::zero();
        m0.m11 = (2.0 * near) / (right - left);
        m0.m12 = 0.0;
        m0.m13 = 0.0;
        m0.m14 = 0.0;
        m0.m22 = (2.0 * near) / (top - bottom);
        m0.m21 = 0.0;
        m0.m23 = 0.0;
        m0.m24 = 0.0;
        m0.m31 = (left + right) / (right - left);
        m0.m32 = (top + bottom) / (top - bottom);
        m0.m33 = far / (near - far);
        m0.m34 = -1.0;
        m0.m43 = (near * far) / (near - far);
        m0.m41 = 0.0;
        m0.m42 = 0.0;
        m0.m44 = 0.0;
        m0
    }
    #[inline(always)]
    pub fn orthographic(width: f32, height: f32, near: f32, far: f32) -> Mat4 {
        let mut m0 = Mat4::zero();
        m0.m11 = 2.0 / width;
        m0.m12 = 0.0;
        m0.m13 = 0.0;
        m0.m14 = 0.0;
        m0.m22 = 2.0 / height;
        m0.m21 = 0.0;
        m0.m23 = 0.0;
        m0.m24 = 0.0;
        m0.m33 = 1.0 / (near - far);
        m0.m31 = 0.0;
        m0.m32 = 0.0;
        m0.m34 = 0.0;
        m0.m41 = 0.0;
        m0.m42 = 0.0;
        m0.m43 = near / (near - far);
        m0.m44 = 1.0;
        m0
    }
    #[inline(always)]
    pub fn orthographic_offset(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Mat4 {
        let mut m0 = Mat4::zero();
        m0.m11 = 2.0 / (right - left);
        m0.m12 = 0.0;
        m0.m13 = 0.0;
        m0.m14 = 0.0;
        m0.m22 = 2.0 / (top - bottom);
        m0.m21 = 0.0;
        m0.m23 = 0.0;
        m0.m24 = 0.0;
        m0.m33 = 1.0 / (near - far);
        m0.m31 = 0.0;
        m0.m32 = 0.0;
        m0.m34 = 0.0;
        m0.m41 = (left + right) / (left - right);
        m0.m42 = (top + bottom) / (bottom - top);
        m0.m43 = near / (near - far);
        m0.m44 = 1.0;
        m0
    }
    #[inline(always)]
    pub fn look_at(position: &Vec3, target: &Vec3, up: &Vec3) -> Mat4 {
        let mut m0 = Mat4::zero();
        let v0 = Vec3::normalize(&Vec3::sub(position, target));
        let v1 = Vec3::normalize(&Vec3::cross(up, &v0));
        let v2 = Vec3::cross(&v0, &v1);
        m0.m11 = v1.x;
        m0.m12 = v2.x;
        m0.m13 = v0.x;
        m0.m14 = 0.0;
        m0.m21 = v1.y;
        m0.m22 = v2.y;
        m0.m23 = v0.y;
        m0.m24 = 0.0;
        m0.m31 = v1.z;
        m0.m32 = v2.z;
        m0.m33 = v0.z;
        m0.m34 = 0.0;
        m0.m41 = -Vec3::dot(&v1, position);
        m0.m42 = -Vec3::dot(&v2, position);
        m0.m43 = -Vec3::dot(&v0, position);
        m0.m44 = 1.0;
        m0
    }
    #[inline(always)]
    pub fn from_quaternion(q0: &Quaternion) -> Mat4 {
        let mut m0 = Mat4::zero();
        let n0 = q0.x * q0.x;
        let n1 = q0.y * q0.y;
        let n2 = q0.z * q0.z;
        let n3 = q0.x * q0.y;
        let n4 = q0.z * q0.w;
        let n5 = q0.z * q0.x;
        let n6 = q0.y * q0.w;
        let n7 = q0.y * q0.z;
        let n8 = q0.x * q0.w;
        m0.m11 = 1.0 - (2.0 * (n1 + n2));
        m0.m12 = 2.0 * (n3 + n4);
        m0.m13 = 2.0 * (n5 - n6);
        m0.m14 = 0.0;
        m0.m21 = 2.0 * (n3 - n4);
        m0.m22 = 1.0 - (2.0 * (n2 + n0));
        m0.m23 = 2.0 * (n7 + n8);
        m0.m24 = 0.0;
        m0.m31 = 2.0 * (n5 + n6);
        m0.m32 = 2.0 * (n7 - n8);
        m0.m33 = 1.0 - (2.0 * (n1 + n0));
        m0.m34 = 0.0;
        m0.m41 = 0.0;
        m0.m42 = 0.0;
        m0.m43 = 0.0;
        m0.m44 = 1.0;
        m0
    }
    #[inline(always)]
    pub fn reflection(p0: Plane) -> Mat4 {
        let mut m0 = Mat4::zero();
        let p1 = Plane::normalize(&p0);
        let x = p1.a;
        let y = p1.b;
        let z = p1.c;
        let n0 = -2.0 * x;
        let n1 = -2.0 * y;
        let n2 = -2.0 * z;
        m0.m11 = (n0 * x) + 1.0;
        m0.m12 = n1 * x;
        m0.m13 = n2 * x;
        m0.m14 = 0.0;
        m0.m21 = n0 * y;
        m0.m22 = (n1 * y) + 1.0;
        m0.m23 = n2 * y;
        m0.m24 = 0.0;
        m0.m31 = n0 * z;
        m0.m32 = n1 * z;
        m0.m33 = (n2 * z) + 1.0;
        m0.m34 = 0.0;
        m0.m41 = n0 * p1.d;
        m0.m42 = n1 * p1.d;
        m0.m43 = n2 * p1.d;
        m0.m44 = 1.0;
        m0
    }
    #[inline(always)]
    pub fn invert(m0: &Mat4) -> Mat4 {
        let mut m1 = Mat4::zero();
        let n0 = m0.m11;
        let n1 = m0.m12;
        let n2 = m0.m13;
        let n3 = m0.m14;
        let n4 = m0.m21;
        let n5 = m0.m22;
        let n6 = m0.m23;
        let n7 = m0.m24;
        let n8 = m0.m31;
        let n9 = m0.m32;
        let n10 = m0.m33;
        let n11 = m0.m34;
        let n12 = m0.m41;
        let n13 = m0.m42;
        let n14 = m0.m43;
        let n15 = m0.m44;
        let n16 = (n10 * n15) - (n11 * n14);
        let n17 = (n9 * n15) - (n11 * n13);
        let n18 = (n9 * n14) - (n10 * n13);
        let n19 = (n8 * n15) - (n11 * n12);
        let n20 = (n8 * n14) - (n10 * n12);
        let n21 = (n8 * n13) - (n9 * n12);
        let n22 = ((n5 * n16) - (n6 * n17)) + (n7 * n18);
        let n23 = -(((n4 * n16) - (n6 * n19)) + (n7 * n20));
        let n24 = ((n4 * n17) - (n5 * n19)) + (n7 * n21);
        let n25 = -(((n4 * n18) - (n5 * n20)) + (n6 * n21));
        let n26 = 1.0 / ((((n0 * n22) + (n1 * n23)) + (n2 * n24)) + (n3 * n25));
        m1.m11 = n22 * n26;
        m1.m21 = n23 * n26;
        m1.m31 = n24 * n26;
        m1.m41 = n25 * n26;
        m1.m12 = -(((n1 * n16) - (n2 * n17)) + (n3 * n18)) * n26;
        m1.m22 = (((n0 * n16) - (n2 * n19)) + (n3 * n20)) * n26;
        m1.m32 = -(((n0 * n17) - (n1 * n19)) + (n3 * n21)) * n26;
        m1.m42 = (((n0 * n18) - (n1 * n20)) + (n2 * n21)) * n26;
        let n27 = (n6 * n15) - (n7 * n14);
        let n28 = (n5 * n15) - (n7 * n13);
        let n29 = (n5 * n14) - (n6 * n13);
        let n30 = (n4 * n15) - (n7 * n12);
        let n32 = (n4 * n14) - (n6 * n12);
        let n33 = (n4 * n13) - (n5 * n12);
        m1.m13 = (((n1 * n27) - (n2 * n28)) + (n3 * n29)) * n26;
        m1.m23 = -(((n0 * n27) - (n2 * n30)) + (n3 * n32)) * n26;
        m1.m33 = (((n0 * n28) - (n1 * n30)) + (n3 * n33)) * n26;
        m1.m43 = -(((n0 * n29) - (n1 * n32)) + (n2 * n33)) * n26;
        let n34 = (n6 * n11) - (n7 * n10);
        let n35 = (n5 * n11) - (n7 * n9);
        let n36 = (n5 * n10) - (n6 * n9);
        let n37 = (n4 * n11) - (n7 * n8);
        let n38 = (n4 * n10) - (n6 * n8);
        let n39 = (n4 * n9) - (n5 * n8);
        m1.m14 = -(((n1 * n34) - (n2 * n35)) + (n3 * n36)) * n26;
        m1.m24 = (((n0 * n34) - (n2 * n37)) + (n3 * n38)) * n26;
        m1.m34 = -(((n0 * n35) - (n1 * n37)) + (n3 * n39)) * n26;
        m1.m44 = (((n0 * n36) - (n1 * n38)) + (n2 * n39)) * n26;
        m1
    }
    #[inline(always)]
    pub fn transpose(m0: &Mat4) -> Mat4 {
        let mut m1 = Mat4::zero();
        m1.m11 = m0.m11;
        m1.m12 = m0.m21;
        m1.m13 = m0.m31;
        m1.m14 = m0.m41;
        m1.m21 = m0.m12;
        m1.m22 = m0.m22;
        m1.m23 = m0.m32;
        m1.m24 = m0.m42;
        m1.m31 = m0.m13;
        m1.m32 = m0.m23;
        m1.m33 = m0.m33;
        m1.m34 = m0.m43;
        m1.m41 = m0.m14;
        m1.m42 = m0.m24;
        m1.m43 = m0.m34;
        m1.m44 = m0.m44;
        m1
    }
    #[inline(always)]
    pub fn determinant(m0: &Mat4) -> f32 {
        let n0 = m0.m11;
        let n1 = m0.m12;
        let n2 = m0.m13;
        let n3 = m0.m14;
        let n4 = m0.m21;
        let n5 = m0.m22;
        let n6 = m0.m23;
        let n7 = m0.m24;
        let n8 = m0.m31;
        let n9 = m0.m32;
        let n10 = m0.m33;
        let n11 = m0.m34;
        let n12 = m0.m41;
        let n13 = m0.m42;
        let n14 = m0.m43;
        let n15 = m0.m44;
        let n16 = (n10 * n15) - (n11 * n14);
        let n17 = (n9 * n15) - (n11 * n13);
        let n18 = (n9 * n14) - (n10 * n13);
        let n19 = (n8 * n15) - (n11 * n12);
        let n20 = (n8 * n14) - (n10 * n12);
        let n21 = (n8 * n13) - (n9 * n12);
        (((n0 * (((n5 * n16) - (n6 * n17)) + (n7 * n18)))
            - (n1 * (((n4 * n16) - (n6 * n19)) + (n7 * n20))))
            + (n2 * (((n4 * n17) - (n5 * n19)) + (n7 * n21))))
            - (n3 * (((n4 * n18) - (n5 * n20)) + (n6 * n21)))
    }
    #[inline(always)]
    pub fn lerp(m0: &Mat4, m1: &Mat4, amount: f32) -> Mat4 {
        let mut m2 = Mat4::zero();
        m2.m11 = m0.m11 + ((m1.m11 - m0.m11) * amount);
        m2.m12 = m0.m12 + ((m1.m12 - m0.m12) * amount);
        m2.m13 = m0.m13 + ((m1.m13 - m0.m13) * amount);
        m2.m14 = m0.m14 + ((m1.m14 - m0.m14) * amount);
        m2.m21 = m0.m21 + ((m1.m21 - m0.m21) * amount);
        m2.m22 = m0.m22 + ((m1.m22 - m0.m22) * amount);
        m2.m23 = m0.m23 + ((m1.m23 - m0.m23) * amount);
        m2.m24 = m0.m24 + ((m1.m24 - m0.m24) * amount);
        m2.m31 = m0.m31 + ((m1.m31 - m0.m31) * amount);
        m2.m32 = m0.m32 + ((m1.m32 - m0.m32) * amount);
        m2.m33 = m0.m33 + ((m1.m33 - m0.m33) * amount);
        m2.m34 = m0.m34 + ((m1.m34 - m0.m34) * amount);
        m2.m41 = m0.m41 + ((m1.m41 - m0.m41) * amount);
        m2.m42 = m0.m42 + ((m1.m42 - m0.m42) * amount);
        m2.m43 = m0.m43 + ((m1.m43 - m0.m43) * amount);
        m2.m44 = m0.m44 + ((m1.m44 - m0.m44) * amount);
        m2
    }
    #[inline(always)]
    pub fn negate(m0: &Mat4) -> Mat4 {
        let mut m1 = Mat4::zero();
        m1.m11 = -m0.m11;
        m1.m12 = -m0.m12;
        m1.m13 = -m0.m13;
        m1.m14 = -m0.m14;
        m1.m21 = -m0.m21;
        m1.m22 = -m0.m22;
        m1.m23 = -m0.m23;
        m1.m24 = -m0.m24;
        m1.m31 = -m0.m31;
        m1.m32 = -m0.m32;
        m1.m33 = -m0.m33;
        m1.m34 = -m0.m34;
        m1.m41 = -m0.m41;
        m1.m42 = -m0.m42;
        m1.m43 = -m0.m43;
        m1.m44 = -m0.m44;
        m1
    }
    #[inline(always)]
    pub fn add(m0: &Mat4, m1: &Mat4) -> Mat4 {
        let mut m2 = Mat4::zero();
        m2.m11 = m0.m11 + m1.m11;
        m2.m12 = m0.m12 + m1.m12;
        m2.m13 = m0.m13 + m1.m13;
        m2.m14 = m0.m14 + m1.m14;
        m2.m21 = m0.m21 + m1.m21;
        m2.m22 = m0.m22 + m1.m22;
        m2.m23 = m0.m23 + m1.m23;
        m2.m24 = m0.m24 + m1.m24;
        m2.m31 = m0.m31 + m1.m31;
        m2.m32 = m0.m32 + m1.m32;
        m2.m33 = m0.m33 + m1.m33;
        m2.m34 = m0.m34 + m1.m34;
        m2.m41 = m0.m41 + m1.m41;
        m2.m42 = m0.m42 + m1.m42;
        m2.m43 = m0.m43 + m1.m43;
        m2.m44 = m0.m44 + m1.m44;
        m2
    }
    #[inline(always)]
    pub fn sub(m0: &Mat4, m1: &Mat4) -> Mat4 {
        let mut m2 = Mat4::zero();
        m2.m11 = m0.m11 - m1.m11;
        m2.m12 = m0.m12 - m1.m12;
        m2.m13 = m0.m13 - m1.m13;
        m2.m14 = m0.m14 - m1.m14;
        m2.m21 = m0.m21 - m1.m21;
        m2.m22 = m0.m22 - m1.m22;
        m2.m23 = m0.m23 - m1.m23;
        m2.m24 = m0.m24 - m1.m24;
        m2.m31 = m0.m31 - m1.m31;
        m2.m32 = m0.m32 - m1.m32;
        m2.m33 = m0.m33 - m1.m33;
        m2.m34 = m0.m34 - m1.m34;
        m2.m41 = m0.m41 - m1.m41;
        m2.m42 = m0.m42 - m1.m42;
        m2.m43 = m0.m43 - m1.m43;
        m2.m44 = m0.m44 - m1.m44;
        m2
    }
    #[inline(always)]
    pub fn mul(m0: &Mat4, m1: &Mat4) -> Mat4 {
        let mut m2 = Mat4::zero();
        m2.m11 = (((m0.m11 * m1.m11) + (m0.m12 * m1.m21)) + (m0.m13 * m1.m31)) + (m0.m14 * m1.m41);
        m2.m12 = (((m0.m11 * m1.m12) + (m0.m12 * m1.m22)) + (m0.m13 * m1.m32)) + (m0.m14 * m1.m42);
        m2.m13 = (((m0.m11 * m1.m13) + (m0.m12 * m1.m23)) + (m0.m13 * m1.m33)) + (m0.m14 * m1.m43);
        m2.m14 = (((m0.m11 * m1.m14) + (m0.m12 * m1.m24)) + (m0.m13 * m1.m34)) + (m0.m14 * m1.m44);
        m2.m21 = (((m0.m21 * m1.m11) + (m0.m22 * m1.m21)) + (m0.m23 * m1.m31)) + (m0.m24 * m1.m41);
        m2.m22 = (((m0.m21 * m1.m12) + (m0.m22 * m1.m22)) + (m0.m23 * m1.m32)) + (m0.m24 * m1.m42);
        m2.m23 = (((m0.m21 * m1.m13) + (m0.m22 * m1.m23)) + (m0.m23 * m1.m33)) + (m0.m24 * m1.m43);
        m2.m24 = (((m0.m21 * m1.m14) + (m0.m22 * m1.m24)) + (m0.m23 * m1.m34)) + (m0.m24 * m1.m44);
        m2.m31 = (((m0.m31 * m1.m11) + (m0.m32 * m1.m21)) + (m0.m33 * m1.m31)) + (m0.m34 * m1.m41);
        m2.m32 = (((m0.m31 * m1.m12) + (m0.m32 * m1.m22)) + (m0.m33 * m1.m32)) + (m0.m34 * m1.m42);
        m2.m33 = (((m0.m31 * m1.m13) + (m0.m32 * m1.m23)) + (m0.m33 * m1.m33)) + (m0.m34 * m1.m43);
        m2.m34 = (((m0.m31 * m1.m14) + (m0.m32 * m1.m24)) + (m0.m33 * m1.m34)) + (m0.m34 * m1.m44);
        m2.m41 = (((m0.m41 * m1.m11) + (m0.m42 * m1.m21)) + (m0.m43 * m1.m31)) + (m0.m44 * m1.m41);
        m2.m42 = (((m0.m41 * m1.m12) + (m0.m42 * m1.m22)) + (m0.m43 * m1.m32)) + (m0.m44 * m1.m42);
        m2.m43 = (((m0.m41 * m1.m13) + (m0.m42 * m1.m23)) + (m0.m43 * m1.m33)) + (m0.m44 * m1.m43);
        m2.m44 = (((m0.m41 * m1.m14) + (m0.m42 * m1.m24)) + (m0.m43 * m1.m34)) + (m0.m44 * m1.m44);
        m2
    }
    #[inline(always)]
    pub fn div(m0: &Mat4, m1: &Mat4) -> Mat4 {
        let mut m2 = Mat4::zero();
        m2.m11 = m0.m11 / m1.m11;
        m2.m12 = m0.m12 / m1.m12;
        m2.m13 = m0.m13 / m1.m13;
        m2.m14 = m0.m14 / m1.m14;
        m2.m21 = m0.m21 / m1.m21;
        m2.m22 = m0.m22 / m1.m22;
        m2.m23 = m0.m23 / m1.m23;
        m2.m24 = m0.m24 / m1.m24;
        m2.m31 = m0.m31 / m1.m31;
        m2.m32 = m0.m32 / m1.m32;
        m2.m33 = m0.m33 / m1.m33;
        m2.m34 = m0.m34 / m1.m34;
        m2.m41 = m0.m41 / m1.m41;
        m2.m42 = m0.m42 / m1.m42;
        m2.m43 = m0.m43 / m1.m43;
        m2.m44 = m0.m44 / m1.m44;
        m2
    }
}

// ------------------------------------------------------------
//
// Equality Operator
//
// ------------------------------------------------------------

impl PartialEq for Mat4 {
    fn eq(&self, rhs: &Self) -> bool {
        Mat4::equals(self, rhs)
    }
}
impl Eq for Mat4 {}

// ------------------------------------------------------------
//
// Operator Overloads: &T + &T
//
// ------------------------------------------------------------

impl Add<&Mat4> for &Mat4 {
    type Output = Mat4;
    fn add(self, rhs: &Mat4) -> Mat4 {
        Mat4::add(self, rhs)
    }
}
impl Sub<&Mat4> for &Mat4 {
    type Output = Mat4;
    fn sub(self, rhs: &Mat4) -> Mat4 {
        Mat4::sub(self, rhs)
    }
}
impl Mul<&Mat4> for &Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: &Mat4) -> Mat4 {
        Mat4::mul(self, rhs)
    }
}
impl Div<&Mat4> for &Mat4 {
    type Output = Mat4;
    fn div(self, rhs: &Mat4) -> Mat4 {
        Mat4::div(self, rhs)
    }
}

// ------------------------------------------------------------
//
// Operator Overloads: T + T
//
// ------------------------------------------------------------

impl Add<Mat4> for Mat4 {
    type Output = Mat4;
    fn add(self, rhs: Mat4) -> Mat4 {
        Mat4::add(&self, &rhs)
    }
}
impl Sub<Mat4> for Mat4 {
    type Output = Mat4;
    fn sub(self, rhs: Mat4) -> Mat4 {
        Mat4::sub(&self, &rhs)
    }
}
impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4::mul(&self, &rhs)
    }
}
impl Div<Mat4> for Mat4 {
    type Output = Mat4;
    fn div(self, rhs: Mat4) -> Mat4 {
        Mat4::div(&self, &rhs)
    }
}

// ------------------------------------------------------------
//
// Display
//
// ------------------------------------------------------------

impl Display for Mat4 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Mat4 {{ m11: {}, m12: {}, m13: {}, m14: {}, m21: {}, m22: {}, m23: {}, m24: {}, m31: {}, m32: {}, m33: {}, m34: {}, m41: {}, m42: {}, m43: {}, m44: {}, }}",
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44,
        )
    }
}
