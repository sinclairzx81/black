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

use black_math::{Vec2};
use std::cmp::{max, min};
use std::mem::swap;

use super::DepthBuffer;
use super::FragmentProgram;
use super::Interpolate;
use super::TargetBuffer;
use super::VertexProgram;

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
        let width       = target.width()  as f32;
        let height      = target.height() as f32;
        let half_width  = width  * 0.5;
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
            return;
        }

        // calculate positions in clip space.
        let clippos_0 = Vec2::new(
            ((position_0.x  / position_0.w) * width) + half_width,
            ((-position_0.y / position_0.w) * height) + half_height,
        );
        let clippos_1 = Vec2::new(
            ((position_1.x  / position_1.w) * width) + half_width,
            ((-position_1.y / position_1.w) * height) + half_height,
        );
        let clippos_2 = Vec2::new(
            ((position_2.x  / position_2.w) * width) + half_width,
            ((-position_2.y / position_2.w) * height) + half_height,
        );

        // run fragment processor
        if Self::edge(&clippos_0, &clippos_1, &clippos_2) >= 0.0 {
            Self::draw_triangle(
                fragment,
                depth,
                target,
                uniform,
                &Interpolate::correct(&varying_0, &position_0.z),
                &Interpolate::correct(&varying_1, &position_1.z),
                &Interpolate::correct(&varying_2, &position_2.z),
                &clippos_0,
                &clippos_1,
                &clippos_2,
                &(1.0 / position_0.z),
                &(1.0 / position_1.z),
                &(1.0 / position_2.z),
            );
        }
    }

    #[inline(always)]
    fn draw_triangle<TTargetBuffer, TFragmentProgram, TVarying, TUniform>(
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
        // clone clippos for sorting.
        let mut ordered_0 = clippos_0.clone();
        let mut ordered_1 = clippos_1.clone();
        let mut ordered_2 = clippos_2.clone();

        // sort ordered y-descending.
        if ordered_0.y > ordered_1.y {
            swap(&mut ordered_0, &mut ordered_1);
        }
        if ordered_1.y > ordered_2.y {
            swap(&mut ordered_1, &mut ordered_2);
        }
        if ordered_0.y > ordered_1.y {
            swap(&mut ordered_0, &mut ordered_1);
        }

        // calculate slopes for the given triangle types.
        //       P0
        //       /|
        //      / | 
        //     /  |
        //    /   |
        // P1 \   | 
        //     \  |
        //      \ |
        //       \|
        //       P2
        let slope_0 = if ordered_1.y - ordered_0.y > 0.0 {
            (ordered_1.x - ordered_0.x) / (ordered_1.y - ordered_0.y)
        } else {
            0.0
        };
        //  P0
        //  |\
        //  | \
        //  |  \
        //  |   \
        //  |   / P1
        //  |  /
        //  | /
        //  |/
        //  P2
        let slope_1 = if ordered_2.y - ordered_0.y > 0.0 {
            (ordered_2.x - ordered_0.x) / (ordered_2.y - ordered_0.y)
        } else {
            0.0
        };

        // draw scanlines
        if slope_0 > slope_1 {
            for y in ordered_0.y as i32..=ordered_2.y as i32 {
                if (y as f32) < ordered_1.y {
                    let (min_x, max_x) = Self::calculate_x_scan_range(
                        y, 
                        &ordered_0,
                        &ordered_2,
                        &ordered_0,
                        &ordered_1,
                    );
                    Self::draw_line(
                        fragment,
                        depth,
                        target,
                        uniform,
                        &clippos_0,
                        &clippos_1,
                        &clippos_2,
                        &varying_0,
                        &varying_1,
                        &varying_2,
                        &corrected_z_0,
                        &corrected_z_1,
                        &corrected_z_2,
                        min_x,
                        max_x,
                        y,
                    )
                } else {
                    let (min_x, max_x) = Self::calculate_x_scan_range(
                        y, 
                        &ordered_0,
                        &ordered_2,
                        &ordered_1,
                        &ordered_2,
                    );
                    Self::draw_line(
                        fragment,
                        depth,
                        target,
                        uniform,
                        &clippos_0,
                        &clippos_1,
                        &clippos_2,
                        &varying_0,
                        &varying_1,
                        &varying_2,
                        &corrected_z_0,
                        &corrected_z_1,
                        &corrected_z_2,
                        min_x,
                        max_x,
                        y,
                    )
                }
            }
        } else {
            for y in ordered_0.y as i32 ..= ordered_2.y as i32 {
                if (y as f32) < ordered_1.y {
                   let (min_x, max_x) = Self::calculate_x_scan_range(
                        y, 
                        &ordered_0,
                        &ordered_1,
                        &ordered_0,
                        &ordered_2,
                    );
                    Self::draw_line(
                        fragment,
                        depth,
                        target,
                        uniform,
                        &clippos_0,
                        &clippos_1,
                        &clippos_2,
                        &varying_0,
                        &varying_1,
                        &varying_2,
                        &corrected_z_0,
                        &corrected_z_1,
                        &corrected_z_2,
                        min_x,
                        max_x,
                        y,
                    )
                } else {
                    let (min_x, max_x) = Self::calculate_x_scan_range(
                        y, 
                        &ordered_1,
                        &ordered_2,
                        &ordered_0,
                        &ordered_2,
                    );
                    Self::draw_line(
                        fragment,
                        depth,
                        target,
                        uniform,
                        &clippos_0,
                        &clippos_1,
                        &clippos_2,
                        &varying_0,
                        &varying_1,
                        &varying_2,
                        &corrected_z_0,
                        &corrected_z_1,
                        &corrected_z_2,
                        min_x,
                        max_x,
                        y,
                    )
                }
            }
        }
    }

    #[inline(always)]
    fn calculate_x_scan_range(y: i32, ordered_0: &Vec2, ordered_1: &Vec2, ordered_2: &Vec2, ordered_3: &Vec2) -> (i32, i32) {
        let gradient_0 = if ordered_0.y != ordered_1.y {
            (y as f32 - ordered_0.y) / (ordered_1.y - ordered_0.y)
        } else {
            1.0
        };
        let gradient_1 = if ordered_2.y != ordered_3.y {
            (y as f32 - ordered_2.y) / (ordered_3.y - ordered_2.y)
        } else {
            1.0
        };
        let min_x = ordered_0.x + (ordered_1.x - ordered_0.x) * Self::clamp(gradient_0, 0.0, 1.0);
        let max_x = ordered_2.x + (ordered_3.x - ordered_2.x) * Self::clamp(gradient_1, 0.0, 1.0);
        (min_x as i32, max_x as i32)
    }

    #[inline(always)]
    fn draw_line<TTargetBuffer, TFragmentProgram, TVarying, TUniform>(
        fragment:      &TFragmentProgram,
        depth:         &mut DepthBuffer,
        target:        &mut TTargetBuffer,
        uniform:       &TUniform,
        clippos_0:     &Vec2,
        clippos_1:     &Vec2,
        clippos_2:     &Vec2,
        varying_0:     &TVarying,
        varying_1:     &TVarying,
        varying_2:     &TVarying,
        corrected_z_0: &f32,
        corrected_z_1: &f32,
        corrected_z_2: &f32,
        min_x:          i32,
        max_x:          i32,
        y:              i32,
    ) where
        TFragmentProgram: FragmentProgram<Uniform = TUniform, Varying = TVarying>,
        TVarying:         Interpolate,
        TTargetBuffer:    TargetBuffer,
    {
        // exit if outside viewport height.
        if y < 0 || y >= target.height() {
            return;
        }
        // min | max within viewport width.
        let min_x = max(min_x, 0);
        let max_x = min(max_x, target.width() - 1);

        // calculate edge value
        let edge = Self::edge(clippos_0, clippos_1, clippos_2);

        for x in min_x..max_x {
            // calculate weights
            let pixel_coordinate = Vec2::new((x as f32) + 0.0, (y as f32) + 0.0);
            let weight_0 = Self::edge(clippos_2, clippos_1, &pixel_coordinate) / edge;
            let weight_1 = Self::edge(clippos_0, clippos_2, &pixel_coordinate) / edge;
            let weight_2 = Self::edge(clippos_1, clippos_0, &pixel_coordinate) / edge;

            // calculate depth of fragment.
            let calculated_depth = 
                  (weight_0 * corrected_z_0)
                + (weight_1 * corrected_z_1)
                + (weight_2 * corrected_z_2);

            // check depth and discard, interpolate and render.
            if calculated_depth < depth.get(x as usize, y as usize) {
                depth.set(x as usize, y as usize, calculated_depth);
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
                target.set(x, y, color);
            }
        }
    }

    #[inline(always)]
    fn clamp(value: f32, min: f32, max: f32) -> f32 {
        min.max(value.min(max))
    }

    #[inline(always)]
    fn edge(v0: &Vec2, v1: &Vec2, v2: &Vec2) -> f32 {
        (v2.x - v0.x) * (v1.y - v0.y) - (v2.y - v0.y) * (v1.x - v0.x)
    }
}
