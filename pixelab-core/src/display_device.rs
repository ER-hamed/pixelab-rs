use crate::{Bitmap, PixelabError};

pub trait DisplayDevice {
    fn create_bitmap(&mut self) -> Box<dyn Bitmap + 'static>;
    fn write(&mut self, bitmap: &mut Box<dyn Bitmap + 'static>) -> Result<(), PixelabError>;
}