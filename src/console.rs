pub trait Writer {
    fn write(&self, value: char);
    fn write_jump(&self);
}

#[derive(Clone, Copy)]
pub struct ConsoleWriter {}

impl ConsoleWriter {
    pub fn new() -> Box<dyn Writer> {
        return Box::new(Self {});
    }
}

impl Writer for ConsoleWriter {
    fn write(&self, value: char) {
        print!("{}", value);
    }

    fn write_jump(&self) {
        self.write('\n');
    }
}
