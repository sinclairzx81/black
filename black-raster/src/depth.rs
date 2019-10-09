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

pub struct DepthBuffer {
    pub data:   Vec<f32>,
    pub width:  usize,
    pub height: usize
}
impl DepthBuffer {
    pub fn new(width: usize, height: usize) -> DepthBuffer {
        let data = vec![0.0; width * height];
        DepthBuffer { width, height, data }
    }
    
    #[inline(always)]
    pub fn clear(&mut self) {
        for n in 0..self.data.len() {
            self.data[n] = std::f32::MAX;
        }
    }
    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, z: f32) {
        self.data[x + y * self.width] = z;
    }
    
    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.data[x + y * self.width]
    }
}