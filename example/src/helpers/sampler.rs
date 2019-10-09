use black::Vec4;

/// A simple checkerboard sampler 
pub struct Sampler {
    frequency: f32
}
impl Sampler {
    pub fn new(frequency: f32) -> Sampler {
        Sampler { frequency }
    }

    #[inline(always)]
    pub fn get(&self, x: f32, y: f32) -> Vec4 {
        let x = if x > 0.999 { 0.999 } else { x };
        let y = if y > 0.999 { 0.999 } else { y };
        
        // let x = if x < 0.0 { 0.0 } else { x };
        // let x = if x < 0.0 { 0.0 } else { x };
        
        let x = (x * self.frequency) as usize;
        let y = (y * self.frequency) as usize;
        let total = x + y;
        if total % 2 == 0 {
            Vec4::new(1.0, 1.0, 1.0, 1.0)
        } else {
            Vec4::new(0.5, 0.5, 0.5, 0.5)
        }
    }
}
