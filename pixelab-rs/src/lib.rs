use pixelab_core::{
    Bitmap, Color, DisplayDevice, Event, EventReader, Handler, PixelabError, Widget,
};

pub struct Screen {
    pub display_device: Box<dyn DisplayDevice>,
    pub bitmap: Box<dyn Bitmap>,
    pub widgets: Vec<Box<dyn Widget>>,
    pub event: EventReader,
    pub background_color: Color,
}

impl Screen {
    pub fn new(mut display_device: Box<dyn DisplayDevice>) -> Self {
        let bitmap = display_device.create_bitmap();
        Self {
            display_device,
            bitmap,
            widgets: vec![],
            event: EventReader::new(),
            background_color: Color::black(),
        }
    }

    pub fn draw(&mut self) -> Result<(), PixelabError> {
        self.bitmap.fill(self.background_color)?;
        for widget in self.widgets.iter_mut() {
            if widget.backend().visible {
                widget.draw(&mut self.bitmap);
                match widget.backend().overlay(&mut self.bitmap) {
                    Ok(_) => {}
                    Err(e) => match e {
                        PixelabError::BitmapOutOfBounds {
                            point,
                            main_bound,
                            sub_bound,
                        } => {
                            return Err(PixelabError::BitmapOutOfBounds {
                                point,
                                main_bound,
                                sub_bound,
                            })
                        }
                        PixelabError::BitmapFormatMismatch => {
                            return Err(PixelabError::BitmapFormatMismatch)
                        }
                        _ => {}
                    },
                };
            }
        }
        self.display_device.write(&mut self.bitmap)?;
        Ok(())
    }

    pub fn add(&mut self, mut widget: impl Widget + 'static) {
        widget.backend().bitmap_type = Some(self.bitmap.create(widget.backend().area));
        widget.init();
        self.widgets.push(Box::new(widget));
    }
    pub fn send_event(&mut self, event: Event, handler: &mut Handler) {
        for widget in self.widgets.iter_mut() {
            widget.event(event.clone(), handler);
        }
    }

    pub fn run<F>(&mut self, mut closure: F) -> Result<(), PixelabError>
    where
        F: FnMut(&mut Self, Event, &mut Handler),
    {
        self.draw()?;
        let mut handler = Handler::new();
        loop {
            self.event.read_all()?;
            match self.event.get() {
                Ok(event) => {
                    self.send_event(event.clone(), &mut handler);
                    closure(self, event.clone(), &mut handler);
                    for message in handler.messages.clone() {
                        self.event.send(Event::Message(message));
                    }
                    if handler.need_update {
                        self.draw()?;
                    }
                    handler.empty();
                }
                Err(_) => {}
            };
        }
    }
}
