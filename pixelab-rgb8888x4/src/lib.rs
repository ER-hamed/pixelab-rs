use pixelab_core::{Area, Bitmap, BitmapFormat, Color, PixelabError, Point};

pub struct RGB8888x4 {
    bitmap: BitmapFormat,
    area: Area,
}
impl RGB8888x4 {
    pub fn new(area: Area) -> Box<Self> {
        let buffer = vec![0; area.width as usize * area.height as usize * 4];
        Box::new(Self {
            bitmap: BitmapFormat::RGB8888x4(buffer),
            area,
        })
    }
}
impl Bitmap for RGB8888x4 {
    fn create(&self, area: Area) -> Box<dyn Bitmap + 'static> {
        let buffer = vec![0; area.width as usize * area.height as usize * 4];
        Box::new(Self {
            bitmap: BitmapFormat::RGB8888x4(buffer),
            area,
        })
    }

    fn set_pixel(&mut self, point: Point, color: Color) -> Result<(), PixelabError> {
        if point.x >= self.area.width || point.y >= self.area.height {
            Err(PixelabError::PointOutOfBounds {
                point,
                bound: self.area,
            })
        } else {
            match &mut self.bitmap {
                BitmapFormat::RGB8888x4(buffer) => {
                    let offset =
                        ((point.y as u32 * self.area.width as u32 + point.x as u32) * 4) as usize;
                    buffer[offset] = color.red;
                    buffer[offset + 1] = color.green;
                    buffer[offset + 2] = color.blue;
                    buffer[offset + 3] = color.alpha;
                    Ok(())
                }
                _ => Err(PixelabError::BitmapFormatMismatch),
            }
        }
    }

    fn fill(&mut self, color: Color) -> Result<(), PixelabError> {
        match &mut self.bitmap {
            BitmapFormat::RGB8888x4(buffer) => {
                let pattern = [color.blue, color.green, color.red, color.alpha];
                let repeated = pattern.repeat(self.area.width as usize * self.area.height as usize);
                buffer.copy_from_slice(&repeated);
                Ok(())
            }
            _ => Err(PixelabError::BitmapFormatMismatch),
        }
    }

    fn overlay(
        &mut self,
        fb: &mut Box<dyn Bitmap + 'static>,
        point: Point,
    ) -> Result<(), PixelabError> {
        if fb.area().width + point.x > self.area.width
            || fb.area().height + point.y > self.area.height
        {
            return Err(PixelabError::BitmapOutOfBounds {
                point,
                main_bound: self.area,
                sub_bound: fb.area().clone(),
            });
        }
        let start_x = point.x.max(0) as usize;
        let start_y = point.y.max(0) as usize;
        let end_x = (point.x + fb.area().width).min(self.area.width) as usize;
        let end_y = (point.y + fb.area().height).min(self.area.height) as usize;
        let width = fb.area().width as usize;
        let big_fb = match fb.buffer() {
            BitmapFormat::RGB8888x4(buffer) => buffer,
            _ => return Err(PixelabError::BitmapFormatMismatch),
        };
        match &mut self.bitmap {
            BitmapFormat::RGB8888x4(buffer) => {
                for y in start_y..end_y {
                    let dst_start = (y * self.area.width as usize + start_x) * 4;
                    let src_start = ((y - start_y) * width + (start_x - point.x as usize)) * 4;
                    let row_width = (end_x - start_x) * 4;
                    unsafe {
                        core::ptr::copy_nonoverlapping(
                            big_fb[src_start..].as_ptr(),
                            buffer[dst_start..].as_mut_ptr(),
                            row_width,
                        );
                    }
                }
            }
            _ => return Err(PixelabError::BitmapFormatMismatch),
        };
        Ok(())
    }

    fn buffer(&mut self) -> &mut BitmapFormat {
        &mut self.bitmap
    }

    fn area(&mut self) -> &mut Area {
        &mut self.area
    }
}
