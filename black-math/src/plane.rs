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

use super::BoundingBox;
use super::Mat4;
use super::Sphere;
use super::Vec3;
use super::Vec4;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum PlaneIntersection {
    Front,
    Back,
    Intersect,
}

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}
impl Plane {
    #[inline(always)]
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Plane {
        Plane { a, b, c, d }
    }
    #[inline(always)]
    pub fn zero() -> Plane {
        Plane::new(0.0, 0.0, 0.0, 0.0)
    }
    #[inline(always)]
    pub fn one() -> Plane {
        Plane::new(1.0, 1.0, 1.0, 1.0)
    }
    #[inline(always)]
    pub fn equals(p0: &Plane, p1: &Plane) -> bool {
        (p0.a == p1.a) && (p0.b == p1.b) && (p0.c == p1.c) && (p0.d == p1.d)
    }
    #[inline(always)]
    pub fn normal(p0: &Plane) -> Vec3 {
        Vec3::new(p0.a, p0.b, p0.c)
    }
    #[inline(always)]
    pub fn normalize(p0: &Plane) -> Plane {
        let n0 = (p0.a * p0.a) + (p0.b * p0.b) + (p0.c * p0.c);
        if f32::abs(n0 - 1.0) < 1.192093E-07 {
            Plane::new(p0.a, p0.b, p0.c, p0.d)
        } else {
            let n1 = 1.0 / f32::sqrt(n0);
            Plane::new(p0.a * n1, p0.b * n1, p0.c * n1, p0.d * n1)
        }
    }
    #[inline(always)]
    pub fn from_points(point1: &Vec3, point2: &Vec3, point3: &Vec3) -> Plane {
        let n0 = point2.x - point1.x;
        let n1 = point2.y - point1.y;
        let n2 = point2.z - point1.z;
        let n3 = point3.x - point1.x;
        let n4 = point3.y - point1.y;
        let n5 = point3.z - point1.z;
        let n6 = (n1 * n5) - (n2 * n4);
        let n7 = (n2 * n3) - (n0 * n5);
        let n8 = (n0 * n4) - (n1 * n3);
        let n9 = ((n6 * n6) + (n7 * n7)) + (n8 * n8);
        let n10 = 1.0 / f32::sqrt(n9);
        let mut p0 = Plane::zero();
        p0.a = n6 * n10;
        p0.b = n7 * n10;
        p0.c = n8 * n10;
        p0.d = -((p0.a * point1.x) + (p0.b * point1.y) + (p0.c * point1.z));
        p0
    }
    #[inline(always)]
    pub fn transform(p0: &Plane, m0: &Mat4) -> Plane {
        let m1 = Mat4::invert(m0);
        let x = p0.a;
        let y = p0.b;
        let z = p0.c;
        let d = p0.d;
        Plane::new(
            (x * m1.m11) + (y * m1.m12) + (z * m1.m13) + (d * m1.m14),
            (x * m1.m21) + (y * m1.m22) + (z * m1.m23) + (d * m1.m24),
            (x * m1.m31) + (y * m1.m32) + (z * m1.m33) + (d * m1.m34),
            (x * m1.m41) + (y * m1.m42) + (z * m1.m43) + (d * m1.m44),
        )
    }
    #[inline(always)]
    pub fn dot4(p0: &Plane, v0: &Vec4) -> f32 {
        (p0.a * v0.x) + (p0.b * v0.y) + (p0.c * v0.z) + (p0.d * v0.w)
    }
    #[inline(always)]
    pub fn dot3(p0: &Plane, v0: &Vec3) -> f32 {
        (p0.a * v0.x) + (p0.b * v0.y) + (p0.c * v0.z) + p0.d
    }
    #[inline(always)]
    pub fn dot_normal(p0: &Plane, n0: &Vec3) -> f32 {
        (p0.a * n0.x) + (p0.b * n0.y) + (p0.c * n0.z)
    }
    #[inline(always)]
    pub fn intersect_box(p0: &Plane, b0: &BoundingBox) -> PlaneIntersection {
        let n0 = Vec3::new(
            if p0.a >= 0.0 { b0.min.x } else { b0.max.x },
            if p0.b >= 0.0 { b0.min.y } else { b0.max.y },
            if p0.c >= 0.0 { b0.min.z } else { b0.max.z },
        );
        let n1 = Vec3::new(
            if p0.a >= 0.0 { b0.max.x } else { b0.min.x },
            if p0.b >= 0.0 { b0.max.y } else { b0.min.y },
            if p0.c >= 0.0 { b0.max.z } else { b0.min.z },
        );
        let mut num = (p0.a * n0.x) + (p0.b * n0.y) + (p0.c * n0.z);
        if (num + p0.d) > 0.0 {
            PlaneIntersection::Front
        } else {
            num = (p0.a * n1.x) + (p0.b * n1.y) + (p0.c * n1.z);
            if (num + p0.d) < 0.0 {
                PlaneIntersection::Back
            } else {
                PlaneIntersection::Intersect
            }
        }
    }
    #[inline(always)]
    pub fn intersect_sphere(plane: &Plane, sphere: &Sphere) -> PlaneIntersection {
        let n0 = (sphere.position.x * plane.a)
            + (sphere.position.y * plane.b)
            + (sphere.position.z * plane.c);
        let n1 = n0 + plane.d;
        if n1 > sphere.radius {
            PlaneIntersection::Front
        } else {
            if n1 < -sphere.radius {
                PlaneIntersection::Back
            } else {
                PlaneIntersection::Intersect
            }
        }
    }
}

// ------------------------------------------------------------
//
// Equality Operator
//
// ------------------------------------------------------------

impl PartialEq for Plane {
    fn eq(&self, rhs: &Self) -> bool {
        Plane::equals(self, rhs)
    }
}
impl Eq for Plane {}


// ------------------------------------------------------------
//
// Display
//
// ------------------------------------------------------------
impl Display for Plane {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Plane {{ a: {}, b: {}, c: {}, d: {} }}",
            self.a, self.b, self.c, self.d
        )
    }
}
