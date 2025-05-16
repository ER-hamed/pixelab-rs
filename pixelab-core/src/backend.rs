use crate::{Area, Bitmap, Color, Handler, PixelabError, Point};

pub struct Backend {
    pub bitmap: Option<Box<dyn Bitmap + 'static>>,
    pub bitmap_type: Option<Box<dyn Bitmap + 'static>>,
    pub position: Point,
    pub area: Area,
    pub enable: bool,
    pub visible: bool,
    pub background_color: Color,
    pub handle_all_event: bool,
    pub on_press: Option<Box<dyn FnMut(Point, &mut Handler, &mut Backend)>>,
    pub on_motion: Option<Box<dyn FnMut(Point, &mut Handler, &mut Backend)>>,
    pub on_release: Option<Box<dyn FnMut(Point, &mut Handler, &mut Backend)>>,
    pub on_message: Option<Box<dyn FnMut(&'static str, &mut Handler, &mut Backend)>>,
}
impl Backend {
    pub fn new() -> Self {
        Self {
            bitmap: None,
            bitmap_type: None,
            position: Point::new(0, 0),
            area: Area::default(),
            enable: true,
            visible: true,
            background_color: Color::white(),
            handle_all_event: true,
            on_press: None,
            on_motion: None,
            on_release: None,
            on_message: None,
        }
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.area.width = width;
        self.area.height = height;
    }

    pub fn create_buffer(&mut self) -> Result<(), PixelabError> {
        match self.bitmap_type.take() {
            Some(bitmap_type) => {
                self.bitmap = Some(bitmap_type.create(self.area));
                self.bitmap_type = Some(bitmap_type);
                Ok(())
            }
            None => Err(PixelabError::BitmapTypeIsNone),
        }
    }

    pub fn destroy_buffer(&mut self) {
        self.bitmap = None
    }

    pub fn overlay(&mut self, fb: &mut Box<dyn Bitmap + 'static>) -> Result<(), PixelabError> {
        match self.bitmap.take() {
            Some(mut buffer) => {
                fb.overlay(&mut buffer, self.position)?;
                self.bitmap = Some(buffer);
                Ok(())
            }
            None => Err(PixelabError::BitmapBufferNotCreated),
        }
    }

    pub fn set_pixel(&mut self, point: Point, color: Color) -> Result<(), PixelabError> {
        match self.bitmap.take() {
            Some(mut buffer) => {
                buffer.set_pixel(point, color)?;
                self.bitmap = Some(buffer);
                Ok(())
            }
            None => Err(PixelabError::BitmapBufferNotCreated),
        }
    }

    pub fn clear(&mut self, color: Color) -> Result<(), PixelabError> {
        match self.bitmap.take() {
            Some(mut buffer) => {
                buffer.fill(color)?;
                self.bitmap = Some(buffer);
                Ok(())
            }
            None => Err(PixelabError::BitmapBufferNotCreated),
        }
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.position.x = x;
        self.position.y = y;
    }
    pub fn check_scope(&mut self, point: Point) -> bool {
        if point.x > self.position.x && point.x < self.position.x + self.area.width {
            if point.y > self.position.y && point.y < self.position.y + self.area.height {
                return true;
            }
        }
        false
    }
    pub fn on_press(&mut self, f: impl 'static + FnMut(Point, &mut Handler, &mut Backend)) {
        self.on_press = Some(Box::new(f));
    }
    pub fn on_motion(&mut self, f: impl 'static + FnMut(Point, &mut Handler, &mut Backend)) {
        self.on_motion = Some(Box::new(f));
    }
    pub fn on_release(&mut self, f: impl 'static + FnMut(Point, &mut Handler, &mut Backend)) {
        self.on_release = Some(Box::new(f));
    }
    pub fn on_message<F>(
        &mut self,
        f: impl 'static + FnMut(&'static str, &mut Handler, &mut Backend),
    ) {
        self.on_message = Some(Box::new(f));
    }
}
