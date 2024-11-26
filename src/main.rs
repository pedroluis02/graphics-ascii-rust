mod console;
mod paint;

use crate::console::Console;
use crate::paint::Paint;

fn main() {
    let paint: Paint = Paint::builder()
        .stroke('#')
        .fill('|')
        .background('-')
        .build();
    println!("{}", paint.to_string());

    let console = Console::new();
    console.write('A');
    console.write('A');
    console.write_jump();
}
