use crate::graphic::{GraphicAscii, GraphicAsciiWritter, GraphicPaintWriter};
use crate::paint::Paint;

pub struct CircleAscii {
    writer: Box<dyn GraphicPaintWriter>,
    radius: i32,
    padding_vertical: i32,
    padding_horizontal: i32,
}

impl CircleAscii {
    pub fn new(radius: i32, paint: Paint) -> Box<dyn GraphicAscii> {
        return Box::new(CircleAscii {
            writer: GraphicAsciiWritter::new(Some(paint), None),
            padding_vertical: 2,
            padding_horizontal: 5,
            radius,
        });
    }

    fn draw_row(&self, index: i32, r: i32, length: i32) {
        let mut is_active = true;
        let mut is_inside = false;

        for x in -length..=length {
            if self.pth(x, index) == r {
                self.writer.write_stroke();

                if is_active && !is_inside {
                    is_inside = true;
                } else if !is_active && is_inside {
                    is_inside = false;
                }
            } else {
                if is_inside && index != r && index != -r {
                    self.writer.write_fill();

                    if is_active {
                        is_active = false;
                    }
                } else {
                    self.writer.write_background();
                }
            }
        }
    }

    fn draw_padding_vertical(&self, length: i32) {
        for _x in 0..self.padding_vertical {
            for _y in 0..(2 * length) {
                self.writer.write_background();
            }
            self.writer.write_jump();
        }
    }

    fn pth(&self, x: i32, y: i32) -> i32 {
        //sqrt(pow(x, 2) + pow(y, 2));
        let x2 = (x * x) as f32;
        let y2 = (y * y) as f32;
        return (x2 + y2).sqrt() as i32;
    }
}

impl GraphicAscii for CircleAscii {
    fn draw(&self) {
        let r = if self.radius < 1 { 10 } else { self.radius };
        let width = r;
        let length = r + (2 * self.padding_horizontal);

        self.draw_padding_vertical(length);

        let mut y = width;
        while y >= -width {
            self.draw_row(y, r, length);
            self.writer.write_jump();

            y -= 2;
        }

        self.draw_padding_vertical(length);
    }
}
