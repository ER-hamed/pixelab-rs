use evdev::{AbsoluteAxisCode, Device, EventSummary, KeyCode};
use pixelab_core::{Event, InputDevice, PixelabError, Point, TouchPad};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub struct TouchPadReader {
    tx: Sender<Event>,
    rx: Receiver<Event>,
    file_name: &'static str,
}
impl TouchPadReader {
    pub fn new(file_name: &'static str) -> Self {
        let (tx, rx) = channel::<Event>();
        Self { tx, rx, file_name }
    }
    fn run(mut file: Device, tx: Sender<Event>) {
        let mut action = TouchPad::Motion(Point::default());
        let mut point = Point::default();
        let mut update = false;
        loop {
            for event in file.fetch_events().unwrap() {
                //println!("{:?}", event.destructure());
                match event.destructure() {
                    EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_X, x) => {
                        if point.x != x as u16 {
                            point.x = x as u16;
                            update = true;
                        }
                    }
                    EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_Y, y) => {
                        if point.y != y as u16 {
                            point.y = y as u16;
                            update = true;
                        }
                    }
                    EventSummary::Key(_, KeyCode::BTN_TOUCH, 1) => {
                        action = TouchPad::Press(Point::default());
                        update = true;
                    }
                    EventSummary::Key(_, KeyCode::BTN_TOUCH, 0) => {
                        action = TouchPad::Release(Point::default());
                        update = true;
                    }
                    _ => {}
                }
            }
            if update {
                match action {
                    TouchPad::Press(_) => tx.send(Event::TouchPad(TouchPad::Press(point))).unwrap(),
                    TouchPad::Motion(_) => {
                        tx.send(Event::TouchPad(TouchPad::Motion(point))).unwrap()
                    }
                    TouchPad::Release(_) => {
                        tx.send(Event::TouchPad(TouchPad::Release(point))).unwrap()
                    }
                }
                action = TouchPad::Motion(Point::default());
                update = false;
            }
            thread::sleep(Duration::from_millis(30));
        }
    }
}
impl InputDevice for TouchPadReader {
    fn init(&mut self) -> Result<(), PixelabError> {
        let tx = self.tx.clone();
        let file = Device::open(self.file_name).expect("");
        thread::spawn(move || Self::run(file, tx));
        Ok(())
    }

    fn read(&mut self) -> Result<Event, PixelabError> {
        match self.rx.try_recv() {
            Ok(event) => Ok(event),
            Err(_) => {
                thread::sleep(Duration::from_millis(10));
                Err(PixelabError::EmptyEvent)
            }
        }
    }
}
