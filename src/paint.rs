#[derive(Debug, PartialEq)]
pub struct Paint {
    stroke: char,
    fill: char,
    background: char,
}

impl Paint {
    pub fn builder() -> PaintBuilder {
        PaintBuilder::new()
    }

    pub fn stroke(&self) -> char {
        self.stroke
    }

    pub fn fill(&self) -> char {
        self.fill
    }

    pub fn background(&self) -> char {
        self.background
    }

    pub fn to_string(&self) -> String {
        format!(
            "Paint{{ stroke: '{}', fill: '{}', bg='{}' }}",
            self.stroke, self.fill, self.background,
        )
    }
}

#[derive(Debug)]
pub struct PaintBuilder {
    paint: Paint,
}

impl PaintBuilder {
    pub fn new() -> PaintBuilder {
        PaintBuilder {
            paint: Paint {
                stroke: '#',
                fill: ' ',
                background: ' ',
            },
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
}

#[test]
fn builder_test() {
    let paint = Paint {
        stroke: '#',
        fill: '|',
        background: '-',
    };
    let paint_from_builder = PaintBuilder::new()
        .stroke('#')
        .fill('|')
        .background('-')
        .build();
    assert_eq!(paint, paint_from_builder);
}
