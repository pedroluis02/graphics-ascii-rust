use crate::console::{ConsoleWriter, Writer};
use crate::paint::Paint;

pub trait GraphicPaintWriter {
    fn write_stroke(&self);
    fn write_fill(&self);
    fn write_background(&self);
    fn write_jump(&self);
}

pub struct GraphicAsciiWritter {
    paint: Paint,
    ouput: Box<dyn Writer>,
}

impl GraphicAsciiWritter {
    pub fn new(
        paint: Option<Paint>,
        ouput: Option<Box<dyn Writer>>,
    ) -> Box<dyn GraphicPaintWriter> {
        return Box::new(GraphicAsciiWritter {
            paint: paint.unwrap_or(Paint::builder().build()),
            ouput: ouput.unwrap_or(ConsoleWriter::new()),
        });
    }
}

impl GraphicPaintWriter for GraphicAsciiWritter {
    fn write_stroke(&self) {
        self.ouput.write(self.paint.stroke())
    }

    fn write_fill(&self) {
        self.ouput.write(self.paint.fill())
    }

    fn write_background(&self) {
        self.ouput.write(self.paint.background())
    }

    fn write_jump(&self) {
        self.ouput.write_jump()
    }
}

pub trait GraphicAscii {
    fn draw(&self);
}
