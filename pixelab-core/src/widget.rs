use crate::{Backend, Bitmap, Event, Handler};

pub trait Widget {
    fn init(&mut self) {}
    fn draw(&mut self, fb: &mut Box<dyn Bitmap + 'static>);
    fn event(&mut self, event: Event, handler: &mut Handler);
    fn backend(&mut self) -> &mut Backend;
}