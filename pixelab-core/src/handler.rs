pub struct Handler {
    pub need_update: bool,
    pub messages: Vec<&'static str>,
}
impl Handler {
    pub fn new() -> Self {
        Self {
            need_update: false,
            messages: vec![],
        }
    }
    pub fn send_message(&mut self, message: &'static str) {
        self.messages.push(message);
    }
    pub fn update(&mut self) {
        self.need_update = true
    }
    pub fn empty(&mut self) {
        self.messages.clear();
        self.need_update = false
    }
}
