mod console;
mod paint;

use crate::console::ConsoleWriter;
use crate::paint::Paint;

fn main() {
    let paint: Paint = Paint::builder()
        .stroke('#')
        .fill('|')
        .background('-')
        .build();
    println!("{}", paint.to_string());

    let console = ConsoleWriter::new();
    console.write('A');
    console.write('A');
    console.write_jump();
}
