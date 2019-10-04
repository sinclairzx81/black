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

use black_math::Vec2;
use std::cmp::{max, min};
use std::mem::swap;

use super::DepthBuffer;
use super::FragmentProgram;
use super::VertexProgram;
use super::Interpolate;
use super::TargetBuffer;

pub struct Raster;
impl Raster {
    #[inline(always)]
    pub fn triangle<TVertexProgram, TFragmentProgram, TUniform, TVertex, TVarying, TTargetBuffer>(
        vertex:   &TVertexProgram,
        fragment: &TFragmentProgram,
        depth:    &mut DepthBuffer,
        target:   &mut TTargetBuffer,
        uniform:  &TUniform,
        vertex_0: &TVertex,
        vertex_1: &TVertex,
        vertex_2: &TVertex,
    ) where
        TVertexProgram:   VertexProgram<Uniform = TUniform, Vertex = TVertex, Varying = TVarying>,
        TFragmentProgram: FragmentProgram<Uniform = TUniform, Varying = TVarying>,
        TVarying:         Interpolate,
        TTargetBuffer:    TargetBuffer,
    {
        // compute half width and height.
        let width       = target.width() as f32;
        let height      = target.height() as f32;
        let half_width  = width * 0.5;
        let half_height = height * 0.5;

        // setup vrs for this primitive.
        let mut varying_0 = Interpolate::new();
        let mut varying_1 = Interpolate::new();
        let mut varying_2 = Interpolate::new();

        // execute vertex shader, store position for interpolation.
        let position_0 = vertex.main(&uniform, &vertex_0, &mut varying_0);
        let position_1 = vertex.main(&uniform, &vertex_1, &mut varying_1);
        let position_2 = vertex.main(&uniform, &vertex_2, &mut varying_2);

        // prevent z less than 0.0 errors, discard the triangle.
        if position_0.z < 0.0 || position_1.z < 0.0 || position_2.z < 0.0 {
            // todo: implement frustum clipping
            return
        }

        // calculate positions in clip space.
        let clippos_0 = Vec2::new(
            f32::floor(((position_0.x / position_0.w) * width) + half_width),
            f32::floor(((-position_0.y / position_0.w) * height) + half_height),
        );
        let clippos_1 = Vec2::new(
            f32::floor(((position_1.x / position_1.w) * width) + half_width),
            f32::floor(((-position_1.y / position_1.w) * height) + half_height),
        );
        let clippos_2 = Vec2::new(
            f32::floor(((position_2.x / position_2.w) * width) + half_width),
            f32::floor(((-position_2.y / position_2.w) * height) + half_height),
        );

        // run fragment processor
        if Self::edge(&clippos_0, &clippos_1, &clippos_2) >= 0.0  {
            Self::run_fragment(
                fragment,
                depth,
                target,
                uniform,
                &Interpolate::correct(&varying_0, &position_0.w),
                &Interpolate::correct(&varying_1, &position_1.w),
                &Interpolate::correct(&varying_2, &position_2.w),
                &clippos_0,
                &clippos_1,
                &clippos_2,
                &(1.0 / position_0.w),
                &(1.0 / position_1.w),
                &(1.0 / position_2.w),
            );
        }
    }

    /// Runs a fragment shader rasterization pass for this triangle.
    #[inline(always)]
    fn run_fragment<TTargetBuffer, TFragmentProgram, TVarying, TUniform>(
        fragment:      &TFragmentProgram,
        depth:         &mut DepthBuffer,
        target:        &mut TTargetBuffer,
        uniform:       &TUniform,
        varying_0:     &TVarying,
        varying_1:     &TVarying,
        varying_2:     &TVarying,
        clippos_0:     &Vec2,
        clippos_1:     &Vec2,
        clippos_2:     &Vec2,
        corrected_z_0: &f32,
        corrected_z_1: &f32,
        corrected_z_2: &f32,
    ) where
        TFragmentProgram: FragmentProgram<Uniform = TUniform, Varying = TVarying>,
        TVarying:         Interpolate,
        TTargetBuffer:    TargetBuffer,
    {
        // clone and sort vertices
        let mut ordered_position_0 = clippos_0.clone();
        let mut ordered_position_1 = clippos_1.clone();
        let mut ordered_position_2 = clippos_2.clone();

        // order the positions in y-descending
        if ordered_position_0.y > ordered_position_1.y {
            swap(&mut ordered_position_0, &mut ordered_position_1);
        }
        if ordered_position_1.y > ordered_position_2.y {
            swap(&mut ordered_position_1, &mut ordered_position_2);
        }
        if ordered_position_0.y > ordered_position_1.y {
            swap(&mut ordered_position_0, &mut ordered_position_1);
        }

        if ordered_position_1.y == ordered_position_2.y {
            Self::render_bottom_flat_triangle(
                fragment,
                depth,
                target,
                uniform,
                &ordered_position_0,
                &ordered_position_1,
                &ordered_position_2,
                varying_0,
                varying_1,
                varying_2,
                clippos_0,
                clippos_1,
                clippos_2,
                corrected_z_0,
                corrected_z_1,
                corrected_z_2,
            );
        } else if ordered_position_0.y == ordered_position_1.y {
            Self::render_top_flat_triangle(
                fragment,
                depth,
                target,
                uniform,
                &ordered_position_0,
                &ordered_position_1,
                &ordered_position_2,
                varying_0,
                varying_1,
                varying_2,
                clippos_0,
                clippos_1,
                clippos_2,
                corrected_z_0,
                corrected_z_1,
                corrected_z_2,
            );
        } else {

            let ordered_position_3 = Vec2::new(
                (ordered_position_0.x
                    + ((ordered_position_1.y - ordered_position_0.y)
                        / (ordered_position_2.y - ordered_position_0.y))
                        * (ordered_position_2.x - ordered_position_0.x)) as i32
                    as f32,
                ordered_position_1.y,
            );

            Self::render_bottom_flat_triangle(
                fragment,
                depth,
                target,
                uniform,
                &ordered_position_0,
                &ordered_position_1,
                &ordered_position_3,
                varying_0,
                varying_1,
                varying_2,
                clippos_0,
                clippos_1,
                clippos_2,
                corrected_z_0,
                corrected_z_1,
                corrected_z_2,
            );
            Self::render_top_flat_triangle(
                fragment,
                depth,
                target,
                uniform,
                &ordered_position_1,
                &ordered_position_3,
                &ordered_position_2,
                varying_0,
                varying_1,
                varying_2,
                clippos_0,
                clippos_1,
                clippos_2,
                corrected_z_0,
                corrected_z_1,
                corrected_z_2,
            );
        }
    }

    #[inline(always)]
    fn render_bottom_flat_triangle<TTargetBuffer, TFragmentProgram, TVarying, TUniform>(
        fragment:           &TFragmentProgram,
        depth:              &mut DepthBuffer,
        target:             &mut TTargetBuffer,
        uniform:            &TUniform,
        ordered_position_0: &Vec2,
        ordered_position_1: &Vec2,
        ordered_position_2: &Vec2,
        varying_0:          &TVarying,
        varying_1:          &TVarying,
        varying_2:          &TVarying,
        clippos_0:          &Vec2,
        clippos_1:          &Vec2,
        clippos_2:          &Vec2,
        corrected_z_0:      &f32,
        corrected_z_1:      &f32,
        corrected_z_2:      &f32,
    ) where
        TFragmentProgram: FragmentProgram<Uniform = TUniform, Varying = TVarying>,
        TVarying:         Interpolate,
        TTargetBuffer:    TargetBuffer,
    {
        let slope_0 = (ordered_position_1.x - ordered_position_0.x)
            / (ordered_position_1.y - ordered_position_0.y);

        let slope_1 = (ordered_position_2.x - ordered_position_0.x)
            / (ordered_position_2.y - ordered_position_0.y);

        let mut cur_x0 = ordered_position_0.x;
        let mut cur_x1 = ordered_position_0.x;

        for cur_y0 in ordered_position_0.y as i32..=ordered_position_1.y as i32 {
            Self::draw_line(
                fragment,
                depth,
                target,
                uniform,
                varying_0,
                varying_1,
                varying_2,
                clippos_0,
                clippos_1,
                clippos_2,
                corrected_z_0,
                corrected_z_1,
                corrected_z_2,
                cur_x0 as i32,
                cur_x1 as i32,
                cur_y0,
            );
            cur_x0 += slope_0;
            cur_x1 += slope_1;
        }
    }

    #[inline(always)]
    fn render_top_flat_triangle<TTargetBuffer, TFragmentProgram, TVarying, TUniform>(
        fragment:           &TFragmentProgram,
        depth:              &mut DepthBuffer,
        target:             &mut TTargetBuffer,
        uniform:            &TUniform,
        ordered_position_0: &Vec2,
        ordered_position_1: &Vec2,
        ordered_position_2: &Vec2,
        varying_0:          &TVarying,
        varying_1:          &TVarying,
        varying_2:          &TVarying,
        clippos_0:          &Vec2,
        clippos_1:          &Vec2,
        clippos_2:          &Vec2,
        corrected_z_0:      &f32,
        corrected_z_1:      &f32,
        corrected_z_2:      &f32,
    ) where
        TFragmentProgram: FragmentProgram<Uniform = TUniform, Varying = TVarying>,
        TVarying:         Interpolate,
        TTargetBuffer:    TargetBuffer,
    {
        let slope_0 = (ordered_position_2.x - ordered_position_0.x)
            / (ordered_position_2.y - ordered_position_0.y);

        let slope_1 = (ordered_position_2.x - ordered_position_1.x)
            / (ordered_position_2.y - ordered_position_1.y);

        let mut cur_x0 = ordered_position_2.x;
        let mut cur_x1 = ordered_position_2.x;

        for cur_y0 in (ordered_position_1.y as i32..=ordered_position_2.y as i32).rev() {
            Self::draw_line(
                fragment,
                depth,
                target,
                uniform,
                varying_0,
                varying_1,
                varying_2,
                clippos_0,
                clippos_1,
                clippos_2,
                corrected_z_0,
                corrected_z_1,
                corrected_z_2,
                cur_x0 as i32,
                cur_x1 as i32,
                cur_y0,
            );
            cur_x0 -= slope_0;
            cur_x1 -= slope_1;
        }
    }

    #[inline(always)]
    fn draw_line<TTargetBuffer, TFragmentProgram, TVarying, TUniform>(
        fragment:      &TFragmentProgram,
        depth:         &mut DepthBuffer,
        target:        &mut TTargetBuffer,
        uniform:       &TUniform,
        varying_0:     &TVarying,
        varying_1:     &TVarying,
        varying_2:     &TVarying,
        clippos_0:     &Vec2,
        clippos_1:     &Vec2,
        clippos_2:     &Vec2,
        corrected_z_0: &f32,
        corrected_z_1: &f32,
        corrected_z_2: &f32,
        cur_x0:        i32,
        cur_x1:        i32,
        cur_y0:        i32,
    ) where
        TFragmentProgram: FragmentProgram<Uniform = TUniform, Varying = TVarying>,
        TVarying:         Interpolate,
        TTargetBuffer:    TargetBuffer,
    {
        // discard fragments outside the Y viewport
        let height = target.height();
        if cur_y0 < 0 || cur_y0 >= height {
            return;
        }

        // discard fragments outside the X viewport
        let width = target.width();
        let minx = max(0, min(cur_x0, cur_x1));
        let maxx = min(width - 1, max(cur_x0, cur_x1));

        // calculate edge value
        let edge = Self::edge(clippos_0, clippos_1, clippos_2);

        // run scanline
        for x in minx..=maxx {
            let weight_0 = Self::edge(clippos_1, clippos_2, &Vec2::new(x as f32, cur_y0 as f32)) / edge;
            let weight_1 = Self::edge(clippos_2, clippos_0, &Vec2::new(x as f32, cur_y0 as f32)) / edge;
            let weight_2 = Self::edge(clippos_0, clippos_1, &Vec2::new(x as f32, cur_y0 as f32)) / edge;

            // calculate depth of fragment.
            let calculated_depth = 
                  (weight_0 * corrected_z_0)
                + (weight_1 * corrected_z_1)
                + (weight_2 * corrected_z_2);

            // check depth and discard, interpolate and render.
            if calculated_depth > depth.get(x as usize, cur_y0 as usize) {
                depth.set(x as usize, cur_y0 as usize, calculated_depth);

                let varying = TVarying::interpolate(
                    varying_0,
                    varying_1,
                    varying_2,
                    &weight_0,
                    &weight_1,
                    &weight_2,
                    &calculated_depth,
                );
                
                let color = fragment.main(uniform, &varying);
                target.set(x, cur_y0, color);
            }
        }
    }

    #[inline(always)]
    fn edge(v0: &Vec2, v1: &Vec2, v2: &Vec2) -> f32 {
        (v2.x - v0.x) * (v1.y - v0.y) - (v2.y - v0.y) * (v1.x - v0.x)
    }
}
