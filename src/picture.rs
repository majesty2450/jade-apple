
#[derive(Debug)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8
}
impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r, g, b
        }
    }
}

pub struct Picture {
    width: u8,
    height: u8,
    pixels: Vec<Pixel>
}
impl Picture {
    pub fn new(width: u8, height: u8, pixels: Vec<Pixel>) -> Self {
        Self {
            width, height, pixels
        }
    }
    pub fn dot(&mut self, x: u8, y: u8, pixel: Pixel) {
        self.pixels[(y * self.width + x) as usize] = pixel;
    }
}

pub struct BlockPicture {
    width: u8,
    height: u8,
    pixels: Vec<Pixel>
}
impl From<Picture> for BlockPicture {
    fn from(pic: Picture) -> Self {
        let mut height = pic.height;
        let mut width = pic.width;
        let mut pixels = pic.pixels;
        if pic.height % 2 == 1 {
            height += 1;
            for i in 0..width {
                pixels.push(Pixel::new(0, 255, 0));
            }
        }
        Self {
            width, height, pixels
        }
    }
}

use crate::printer::*;

impl Print for BlockPicture {
    fn print(&self, printer: &mut Printer) {
        // printer.repos(0, 0);
        for row in (0..self.height).step_by(2) {
            for col in 0..self.width {
                let findex = ((row as u16 * self.width as u16) + col as u16) as usize;
                printer.color_fg_rgb(self.pixels[findex].r, self.pixels[findex].g, self.pixels[findex].b);
                let bindex = (((row as u16 + 1 as u16) * self.width as u16) + col as u16) as usize;
                printer.color_bg_rgb(self.pixels[bindex].r, self.pixels[bindex].g, self.pixels[bindex].b);
                printer.write("▀");
            }
            printer.newline();
        }
    }
}