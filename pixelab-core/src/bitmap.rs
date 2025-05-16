use crate::{Area, Color, PixelabError, Point};

pub enum BitmapFormat {
    RGB8888x4(Vec<u8>),
    RGB565x1(Vec<u16>),
    RGB8888x1(Vec<u32>),
}

pub trait Bitmap {
    fn create(&self, area: Area) -> Box<dyn Bitmap + 'static>;
    fn set_pixel(&mut self, point: Point, color: Color) -> Result<(), PixelabError>;
    fn fill(&mut self, color: Color) -> Result<(), PixelabError>;
    fn overlay(
        &mut self,
        fb: &mut Box<dyn Bitmap + 'static>,
        point: Point,
    ) -> Result<(), PixelabError>;
    fn buffer(&mut self) -> &mut BitmapFormat;
    fn area(&mut self) -> &mut Area;
    fn draw_line(
        &mut self,
        start_point: Point,
        end_point: Point,
        color: Color,
    ) -> Result<(), PixelabError> {
        let x0 = start_point.x as i32;
        let y0 = start_point.y as i32;
        let x1 = end_point.x as i32;
        let y1 = end_point.y as i32;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut x = x0;
        let mut y = y0;
        let mut err = dx - dy;
        loop {
            if x >= 0 && x < self.area().width as i32 && y >= 0 && y < self.area().height as i32 {
                let point = Point::new(x as u16, y as u16);
                self.set_pixel(point, color)?;
            }
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
        Ok(())
    }
    fn set_border(&mut self, color: Color) -> Result<(), PixelabError> {
        let width = self.area().width as usize;
        let height = self.area().height as usize;
        for x in 0..width {
            if x < width {
                self.set_pixel(Point::new(x as u16, 0), color)?;
            }
        }
        for x in 0..width {
            if x < width {
                self.set_pixel(Point::new(x as u16, (height - 1) as u16), color)?;
            }
        }
        for y in 1..height - 1 {
            if y < height {
                self.set_pixel(Point::new(0, y as u16), color)?;
            }
        }
        for y in 1..height - 1 {
            if y < height {
                self.set_pixel(Point::new((width - 1) as u16, y as u16), color)?;
            }
        }
        Ok(())
    }
    fn draw_poly_line(&mut self, points: Vec<(Point, Color)>) -> Result<(), PixelabError> {
        if points.len() < 2 {
            return Err(PixelabError::InsufficientPoints);
        }
        for i in 0..points.len() - 1 {
            let start = points[i].0;
            let end = points[i + 1].0;
            let color = points[i].1;
            self.draw_line(start, end, color)?;
        }
        Ok(())
    }
}
