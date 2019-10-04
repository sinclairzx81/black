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
use super::Plane;
use super::Ray;
use super::Vec3;
use std::fmt::{Display, Error, Formatter};

#[inline(always)]
fn compute_intersection_ray(p0: &Plane, p1: &Plane) -> Ray {
    let n0 = Plane::normal(&p0);
    let n1 = Plane::normal(&p1);
    let v0 = Vec3::cross(&n0, &n1);
    let num = Vec3::length_sq(&v0);
    let v1 = Vec3::scale(&n1, -p0.d);
    let v2 = Vec3::scale(&n0, p1.d);
    let v3 = Vec3::add(&v1, &v2);
    let v4 = Vec3::cross(&v3, &v0);
    let v5 = Vec3::new(v4.x / num, v4.y / num, v4.z / num);
    Ray::new(v5, v0)
}

#[inline(always)]
fn compute_intersection_vector(plane: &Plane, ray: &Ray) -> Vec3 {
    let n0 = Plane::normal(&plane);
    let num = (-plane.d - Vec3::dot(&n0, &ray.position)) / Vec3::dot(&n0, &ray.direction);
    Vec3::add(&ray.position, &Vec3::scale(&ray.direction, num))
}

#[derive(Debug, Clone)]
pub struct Frustum {
    pub near: Plane,
    pub far: Plane,
    pub left: Plane,
    pub right: Plane,
    pub top: Plane,
    pub bottom: Plane,
    pub corners: Vec<Vec3>,
}
impl Frustum {
    /// Constructs a new Frustum from the given projection Mat4.
    pub fn new(m0: &Mat4) -> Frustum {
        let mut planes = vec![
            // near
            Plane::new(-m0.m13, -m0.m23, -m0.m33, -m0.m43),
            // far
            Plane::new(
                -m0.m14 + m0.m13,
                -m0.m24 + m0.m23,
                -m0.m34 + m0.m33,
                -m0.m44 + m0.m43,
            ),
            // left
            Plane::new(
                -m0.m14 - m0.m11,
                -m0.m24 - m0.m21,
                -m0.m34 - m0.m31,
                -m0.m44 - m0.m41,
            ),
            // right
            Plane::new(
                -m0.m14 + m0.m11,
                -m0.m24 + m0.m21,
                -m0.m34 + m0.m31,
                -m0.m44 + m0.m41,
            ),
            // top
            Plane::new(
                -m0.m14 + m0.m12,
                -m0.m24 + m0.m22,
                -m0.m34 + m0.m32,
                -m0.m44 + m0.m42,
            ),
            // bottom
            Plane::new(
                -m0.m14 - m0.m12,
                -m0.m24 - m0.m22,
                -m0.m34 - m0.m32,
                -m0.m44 - m0.m42,
            ),
        ];
        for plane in planes.iter_mut() {
            let len = Vec3::length(&Plane::normal(plane));
            plane.a = plane.a / len;
            plane.b = plane.b / len;
            plane.c = plane.c / len;
            plane.d = plane.d / len;
        }
        let mut corners: Vec<Vec3> = Vec::with_capacity(8);
        let mut ray = compute_intersection_ray(&planes[0], &planes[2]);
        corners[0] = compute_intersection_vector(&planes[4], &ray);
        corners[3] = compute_intersection_vector(&planes[5], &ray);
        ray = compute_intersection_ray(&planes[3], &planes[0]);
        corners[1] = compute_intersection_vector(&planes[4], &ray);
        corners[2] = compute_intersection_vector(&planes[5], &ray);
        ray = compute_intersection_ray(&planes[2], &planes[1]);
        corners[4] = compute_intersection_vector(&planes[4], &ray);
        corners[7] = compute_intersection_vector(&planes[5], &ray);
        ray = compute_intersection_ray(&planes[1], &planes[3]);
        corners[5] = compute_intersection_vector(&planes[4], &ray);
        corners[6] = compute_intersection_vector(&planes[5], &ray);
        Frustum {
            near: planes[0].clone(),
            far: planes[1].clone(),
            left: planes[2].clone(),
            right: planes[3].clone(),
            top: planes[4].clone(),
            bottom: planes[5].clone(),
            corners,
        }
    }
    #[inline(always)]
    pub fn equals(f0: &Frustum, f1: &Frustum) -> bool {
        Plane::equals(&f0.near, &f1.near)
            && Plane::equals(&f0.far, &f1.far)
            && Plane::equals(&f0.left, &f1.left)
            && Plane::equals(&f0.right, &f1.right)
            && Plane::equals(&f0.top, &f1.top)
            && Plane::equals(&f0.bottom, &f1.bottom)
    }
}

// ------------------------------------------------------------
//
// Equality Operator
//
// ------------------------------------------------------------

impl PartialEq for Frustum {
    fn eq(&self, rhs: &Self) -> bool {
        Frustum::equals(self, rhs)
    }
}
impl Eq for Frustum {}

// ------------------------------------------------------------
//
// Display
//
// ------------------------------------------------------------

impl Display for Frustum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Frustum {{ near: {}, far: {}, left: {}, right: {}, top: {}, bottom: {} }}",
            self.near, self.far, self.left, self.right, self.top, self.bottom
        )
    }
}
