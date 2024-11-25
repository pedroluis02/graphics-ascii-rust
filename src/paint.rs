#[derive(Default)]
pub struct Paint {
    stroke: char,
    fill: char,
    background: char,
}

impl Paint {
    pub fn to_string(self) -> String {
        format!(
            "Paint{{ stroke: '{}', fill: '{}', bg='{}' }}",
            self.stroke, self.fill, self.background,
        )
    }
}

#[derive(Default)]
pub struct PaintBuilder {
    paint: Paint,
}

impl PaintBuilder {
    pub fn new() -> PaintBuilder {
        PaintBuilder {
            paint: Self::default_paint(),
        }
    }

    pub fn stroke(mut self, value: char) -> PaintBuilder {
        self.paint.stroke = value;
        self
    }

    pub fn fill(mut self, value: char) -> PaintBuilder {
        self.paint.fill = value;
        self
    }

    pub fn background(mut self, value: char) -> PaintBuilder {
        self.paint.background = value;
        self
    }

    pub fn build(self) -> Paint {
        self.paint
    }

    fn default_paint() -> Paint {
        Paint {
            stroke: '#',
            fill: ' ',
            background: ' ',
        }
    }
}
