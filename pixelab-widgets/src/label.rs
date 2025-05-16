use pixelab_core::{Backend, Bitmap, Event, Handler, Widget};

pub struct Label {
    pub backend: Backend,
    on_event: Option<Box<dyn FnMut(&mut Label, Event, &mut Handler)>>,
}
impl Label {
    pub fn new() -> Self {
        Self {
            backend: Backend::new(),
            on_event: None,
        }
    }
    pub fn on_event(&mut self, f: impl 'static + FnMut(&mut Self, Event, &mut Handler)) {
        self.on_event = Some(Box::new(f));
    }
    pub fn handle_on_event(&mut self, event: Event, handler: &mut Handler) {
        if let Some(mut f) = self.on_event.take() {
            f(self, event, handler);
            self.on_event = Some(f)
        }
    }
}
impl Widget for Label {
    fn init(&mut self) {
        self.backend.create_buffer().unwrap();
        self.backend.clear(self.backend.background_color).unwrap();
    }

    fn draw(&mut self, _fb: &mut Box<dyn Bitmap + 'static>) {
        //println!("draw")
    }

    fn event(&mut self, event: Event, handler: &mut Handler) {
        self.handle_on_event(event, handler);
    }

    fn backend(&mut self) -> &mut Backend {
        &mut self.backend
    }
}
