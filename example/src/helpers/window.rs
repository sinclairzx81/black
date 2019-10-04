use black::{ TargetBuffer, Vec4 };
use minifb::{ Key, Window, WindowOptions, Scale };
use std::mem;

/// ---------------------------------------------------
/// Pixel buffer
/// ---------------------------------------------------
pub struct Buffer {
    pub width:  usize,
    pub height: usize,
    pub data:   Vec<u32>
}
impl Buffer {
    pub fn new(width: usize, height: usize) -> Buffer {
        Buffer {width, height, data: vec![0; width * height] }
    }
    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, c: u32) {
        self.data[x + (y * self.width)] = c;
    }
}

/// ---------------------------------------------------
/// Errors
/// ---------------------------------------------------
#[derive(Debug)]
pub struct ContextError {
    pub message: String,
}
impl ContextError {
    fn new(message: &str) -> ContextError {
        ContextError {
            message: message.to_string(),
        }
    }
}

/// ---------------------------------------------------
/// Builder
/// ---------------------------------------------------

#[derive(Clone)]
pub struct Builder {
    pub title:      String,
    pub width:      usize,
    pub height:     usize,
    pub pixel_size:  usize,
    pub x:          isize,
    pub y:          isize
}
impl Builder {
    pub fn default() -> Builder {
        Builder { 
            title:      "".to_string(),
            width:      512,
            height:     512,
            pixel_size: 1,
            x:          100,
            y:          100
        }
    }
    #[allow(dead_code)]
    pub fn title(&self, title: &str) -> Builder {
        let mut clone = self.clone();
        clone.title = title.to_owned();
        clone
    }
    #[allow(dead_code)]
    pub fn size(&self, width: usize, height: usize) -> Builder {
        let mut clone = self.clone();
        clone.width  = width;
        clone.height = height;
        clone
    }
    #[allow(dead_code)]
    pub fn pixel_size(&self, size: usize) -> Builder {
        let mut clone = self.clone();
        clone.pixel_size = size;
        clone
    }
    #[allow(dead_code)]
    pub fn position(&self, x: isize, y: isize) -> Builder {
        let mut clone = self.clone();
        clone.x = x;
        clone.y = y;
        clone
    }
    #[allow(dead_code)]
    pub fn create(self) -> Result<Context, ContextError> {
        let buffer = Buffer::new(self.width, self.height);
        let mut window = Window::new(
             self.title.as_ref(),
             self.width,
             self.height,
             WindowOptions {
                 scale: match self.pixel_size {
                     1 => Scale::X1,
                     2 => Scale::X2,
                     4 => Scale::X4,
                     8 => Scale::X8,
                     16 => Scale::X16,
                     32 => Scale::X32,
                     _ =>  {
                         return Err(
                             ContextError::new("Only support pixel sizes of 1, 2, 4, 8, 16, and 32")
                             )
                     }
                 },
                 ..WindowOptions::default()
             },
        )
        .map_err(|_| ContextError::new("Unable not initialize window"))?;
        window.set_position(self.x, self.y);
        Ok(Context::new(window, buffer))
    }
}

/// ---------------------------------------------------
/// Context
/// ---------------------------------------------------
pub struct Context {
    window:  Window,
    buffer:  Buffer,
}
impl Context {
    pub fn new(window: Window, buffer: Buffer) -> Context {
        Context { window, buffer }
    }
    pub fn clear(&mut self, color: u32) {
        for x in 0..self.buffer.data.len() {
            self.buffer.data[x] = color;
        }
    }
    pub fn active(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn present(&mut self) -> Result<(), ContextError> {
        self.window
            .update_with_buffer(&self.buffer.data)
            .map_err(|_| ContextError::new("Unable to present buffer to window."))
    }
    #[inline(always)]
    fn clamp(&mut self, n: f32) -> f32 {
        if n < 0.0 { return 0.0; }
        if n > 1.0 { return 1.0; }
        n
    }
}

/// ---------------------------------------------------
/// Target implementation
/// ---------------------------------------------------
impl TargetBuffer for Context {
    
    #[inline(always)]
    fn width(&self) -> i32 {
        self.buffer.width as i32
    }
    
    #[inline(always)]
    fn height(&self) -> i32 {
        self.buffer.height as i32
    }
    
    #[inline(always)]
    fn set(&mut self, x: i32, y: i32, color: Vec4) {
        unsafe {
            let s = [
                (self.clamp(color.z) * 255.0) as u8, 
                (self.clamp(color.y) * 255.0) as u8, 
                (self.clamp(color.x) * 255.0) as u8, 
                (self.clamp(color.w) * 255.0) as u8
            ];
            let c = mem::transmute::<[u8; 4], u32>(s);
            self.buffer.set(x as usize, y as usize, c)
        }
    }

}

