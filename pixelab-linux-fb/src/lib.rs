use pixelab_core::{Area, Bitmap, BitmapFormat, DisplayDevice, PixelabError};
use pixelab_rgb8888x1::RGB8888x1;
use std::fs::File;
use std::io::{self, Seek, SeekFrom, Write};
use pixelab_rgb8888x4::RGB8888x4;

pub struct LinuxFB {
    file: File,
    area: Area,
    bitmap: BitmapFormat,
}
impl LinuxFB {
    pub fn new_rgb8888x1(
        file: &'static str,
        width: u16,
        height: u16,
    ) -> Result<Box<Self>, io::Error> {
        let file = File::create(file)?;
        Ok(Box::new(Self {
            file,
            area: Area::new(width, height),
            bitmap: BitmapFormat::RGB8888x1(vec![]),
        }))
    }
    pub fn new_rgb8888x4(
        file: &'static str,
        width: u16,
        height: u16,
    ) -> Result<Box<Self>, io::Error> {
        let file = File::create(file)?;
        Ok(Box::new(Self {
            file,
            area: Area::new(width, height),
            bitmap: BitmapFormat::RGB8888x4(vec![]),
        }))
    }
}
impl DisplayDevice for LinuxFB {
    fn create_bitmap(&mut self) -> Box<dyn Bitmap + 'static> {
        match self.bitmap {
            BitmapFormat::RGB8888x1(_) => {
                RGB8888x1::new(self.area)
            }
            BitmapFormat::RGB8888x4(_) => {
                RGB8888x4::new(self.area)
            }
            _ => panic!()
        }
    }

    fn write(&mut self, bitmap: &mut Box<dyn Bitmap + 'static>) -> Result<(), PixelabError> {
        match &bitmap.buffer() {
            BitmapFormat::RGB8888x4(bitmap) => {
                self.file.seek(SeekFrom::Start(0)).unwrap();
                self.file.write(bitmap).unwrap();
                Ok(())
            }
            BitmapFormat::RGB8888x1(bitmap) => {
                self.file.seek(SeekFrom::Start(0)).unwrap();
                self.file
                    .write(unsafe {
                        core::slice::from_raw_parts(
                            bitmap.as_ptr() as *const u8,
                            bitmap.len() * size_of::<u32>(),
                        )
                    })
                    .unwrap();

                Ok(())
            }
            _ => Err(PixelabError::BitmapFormatMismatch),
        }
    }
}
