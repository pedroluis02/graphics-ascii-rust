mod paint;

use crate::paint::Paint;
use crate::paint::PaintBuilder;

fn main() {
    let paint: Paint = PaintBuilder::new()
        .stroke('#')
        .fill('|')
        .background('-')
        .build();
    println!("{}", paint.to_string());
}
