use crate::{PixelabError, Point};
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub enum TouchPad {
    Press(Point),
    Motion(Point),
    Release(Point),
}

#[derive(Clone, Debug)]
pub enum Keyboard {
    Press(char),
    Release(char),
}

#[derive(Clone, Debug)]
pub enum Mouse {
    Press(u8),
    Motion(Point),
    Release(u8),
}

#[derive(Clone, Debug)]
pub enum Event {
    TouchPad(TouchPad),
    Keyboard(Keyboard),
    Mouse(Mouse),
    KeyValue(&'static str, u32),
    Message(&'static str),
    Timer(u8),
}
impl core::fmt::Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait InputDevice {
    fn init(&mut self) -> Result<(), PixelabError>;
    fn read(&mut self) -> Result<Event, PixelabError>;
}

pub struct EventReader {
    pub readers: Vec<Box<dyn InputDevice>>,
    pub events: Vec<Event>,
}
impl EventReader {
    pub fn new() -> Self {
        Self {
            readers: vec![],
            events: vec![],
        }
    }
    pub fn add_reader(
        &mut self,
        mut reader: impl InputDevice + 'static,
    ) -> Result<(), PixelabError> {
        reader.init()?;
        self.readers.push(Box::new(reader));
        Ok(())
    }
    pub fn read_all(&mut self) -> Result<(), PixelabError> {
        for reader in self.readers.iter_mut() {
            match reader.read() {
                Ok(event) => self.events.push(event),
                Err(_) => {}
            }
        }
        Ok(())
    }
    pub fn get(&mut self) -> Result<Event, PixelabError> {
        match self.events.pop() {
            Some(event) => Ok(event),
            None => Err(PixelabError::EmptyEvent),
        }
    }
    pub fn send(&mut self, event: Event) {
        self.events.push(event)
    }
}
