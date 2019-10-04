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
use super::Plane;
use super::Sphere;
use super::Triangle;
use super::Vec3;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub position: Vec3,
    pub direction: Vec3,
}
impl Ray {
    pub fn new(position: Vec3, direction: Vec3) -> Ray {
        Ray {
            position,
            direction,
        }
    }
    #[inline(always)]
    pub fn equals(r0: &Ray, r1: &Ray) -> bool {
        Vec3::equals(&r0.position, &r1.position) && Vec3::equals(&r0.direction, &r1.direction)
    }
    #[inline(always)]
    pub fn intersect_plane(ray: Ray, plane: Plane) -> Option<f32> {
        let n0 =
            (plane.a * ray.direction.x) + (plane.b * ray.direction.y) + (plane.c * ray.direction.z);
        if f32::abs(n0) < 1E-05 {
            None
        } else {
            let n1 = (plane.a * ray.position.x)
                + (plane.b * ray.position.y)
                + (plane.c * ray.position.z);
            let mut n2 = (-plane.d - n1) / n0;
            if n2 < 0.0 {
                if n2 < -1E-05 {
                    return None;
                }
                n2 = 0.0;
            }
            Some(n2)
        }
    }
    #[inline(always)]
    pub fn intersect_triangle(ray: &Ray, triangle: &Triangle) -> Option<f32> {
        let v0 = Vec3::sub(&triangle.v1, &triangle.v0);
        let v1 = Vec3::sub(&triangle.v2, &triangle.v0);
        let v2 = Vec3::cross(&ray.direction, &v1);
        let n0 = Vec3::dot(&v0, &v2);
        if n0 > -0.00001 {
            return None;
        }
        let n1 = 1.0 / n0;
        let v3 = Vec3::sub(&ray.position, &triangle.v0);
        let n2 = Vec3::dot(&v3, &v2) * n1;
        if n2 < -0.001 || n2 > 1.001 {
            return None;
        }

        let v4 = Vec3::cross(&v3, &v0);
        let n3 = Vec3::dot(&ray.direction, &v4) * n1;
        if n3 < -0.001 || n2 + n3 > 1.001 {
            return None;
        }
        let x = Vec3::dot(&v1, &v4) * n1;
        if x <= 0.0 {
            None
        } else {
            Some(x)
        }
    }

    #[inline(always)]
    pub fn intersect_box(r0: &Ray, b0: &BoundingBox) -> Option<f32> {
        let mut max_value = std::f32::MAX;
        let mut result = 0.0;
        if f32::abs(r0.direction.x) < 1E-06 {
            if (r0.position.x < b0.min.x) || (r0.position.x > b0.max.x) {
                return None;
            }
        } else {
            let n0 = 1.0 / r0.direction.x;
            let mut n1 = (b0.min.x - r0.position.x) * n0;
            let mut n2 = (b0.max.x - r0.position.x) * n0;
            if n1 > n2 {
                let n3 = n1;
                n1 = n2;
                n2 = n3;
            }
            result = if n1 > result { n1 } else { result };
            max_value = if n2 < result { n2 } else { result };
            if result > max_value {
                return None;
            }
        }
        if f32::abs(r0.direction.y) < 1E-06 {
            if (r0.position.y < b0.min.y) || (r0.position.y > b0.max.y) {
                return None;
            }
        } else {
            let n0 = 1.0 / r0.direction.y;
            let mut n1 = (b0.min.y - r0.position.y) * n0;
            let mut n2 = (b0.max.y - r0.position.y) * n0;
            if n1 > n2 {
                let n3 = n1;
                n1 = n2;
                n2 = n3;
            }
            result = if n1 > result { n1 } else { result };
            max_value = if n2 < max_value { n2 } else { max_value };
            if result > max_value {
                return None;
            }
        }
        if f32::abs(r0.direction.z) < 1E-06 {
            if (r0.position.z < b0.min.z) || (r0.position.z > b0.max.z) {
                return None;
            }
        } else {
            let n0 = 1.0 / r0.direction.z;
            let mut n1 = (b0.min.z - r0.position.z) * n0;
            let mut n2 = (b0.max.z - r0.position.z) * n0;
            if n1 > n2 {
                let n3 = n1;
                n1 = n2;
                n2 = n3;
            }
            result = if n1 > result { n1 } else { result };
            max_value = if n2 < max_value { n2 } else { max_value };
            if result > max_value {
                return None;
            }
        }
        return Some(result);
    }

    #[inline(always)]
    pub fn intersect_sphere(r0: &Ray, s0: &Sphere) -> Option<f32> {
        let n0 = s0.position.x - r0.position.x;
        let n1 = s0.position.y - r0.position.y;
        let n2 = s0.position.z - r0.position.z;
        let n3 = (n0 * n0) + (n1 * n1) + (n2 * n2);
        let n4 = s0.radius * s0.radius;
        if n3 <= n4 {
            return Some(0.0);
        }
        let n5 = (n0 * r0.direction.x) + (n1 * r0.direction.y) + (n2 * r0.direction.z);
        if n5 < 0.0 {
            return None;
        }
        let n6 = n3 - (n5 * n5);
        if n6 > n4 {
            return None;
        }
        let n7 = f32::sqrt(n4 - n6);
        Some(n5 - n7)
    }
}

// ------------------------------------------------------------
//
// Equality Operator
//
// ------------------------------------------------------------

impl PartialEq for Ray {
    fn eq(&self, rhs: &Self) -> bool {
        Ray::equals(self, rhs)
    }
}
impl Eq for Ray {}

// ------------------------------------------------------------
//
// Display
//
// ------------------------------------------------------------

impl Display for Ray {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Ray {{ position: {}, direction: {} }}",
            self.position, self.direction
        )
    }
}
