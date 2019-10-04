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

extern crate black_codegen;
pub use black_codegen::*;

extern crate black_math   as math;
extern crate black_raster as raster;

pub use {
    math::Mat4,
    math::BoundingBox,
    math::Frustum,
    math::Plane,
    math::Vec2,
    math::Vec3,
    math::Vec4,
    math::Quaternion,
    math::Ray,
    math::Sphere,
    math::Triangle,
};

pub use {
    raster::FragmentProgram,
    raster::VertexProgram,
    raster::Interpolate,
    raster::DepthBuffer,
    raster::TargetBuffer,
    raster::Raster,
};