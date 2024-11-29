mod circle;
mod console;
mod graphic;
mod paint;

use crate::circle::CircleAscii;
use crate::paint::Paint;

fn main() {
    let paint: Paint = Paint::builder()
        .stroke('#')
        .fill('|')
        .background('-')
        .build();
    println!("{}", paint.to_string());

    let graphic = CircleAscii::new(10, paint);
    graphic.draw();
}
